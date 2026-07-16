//! `agileplus dag` — orchestrate a subagent DAG session.
//!
//! Subcommands
//! -----------
//! * `run <name>`      — create a new session and run it to completion
//! * `resume <name>`   — resume an incomplete session
//! * `check <name>`    — print status summary (no execution)
//! * `delete <name>`   — remove a session from the store

use agileplus_dag_orchestrator::dag::{Dag, Shard};
use agileplus_dag_orchestrator::dispatch::{
    build_agent_prompt, Dispatcher, NoopDispatcher, ProcessDispatcher,
    ProcessConfig,
};
use agileplus_dag_orchestrator::executor::Executor;
use agileplus_dag_orchestrator::progress::ProgressReport;
use agileplus_dag_orchestrator::session::SessionStore;
use agileplus_dag_orchestrator::task::{Task, TaskId, TaskStatus, Priority};
use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use std::sync::Arc;

#[derive(Debug, Args)]
pub struct DagCommand {
    #[command(subcommand)]
    pub sub: DagSub,
}

#[derive(Debug, Subcommand)]
pub enum DagSub {
    /// Create a new session with a real DAG and run it to completion.
    Run {
        /// Session / DAG name.
        name: String,
        /// Path to the SQLite store.
        #[arg(short = 'd', long, default_value = "dag.db")]
        db: String,
        /// Max parallel shards per level.
        #[arg(long, default_value = "2")]
        max_parallel: usize,
        /// Agent binary to use (default: noop / dry-run).
        #[arg(long)]
        agent: Option<String>,
        /// Arguments for the agent binary.
        #[arg(long)]
        agent_args: Vec<String>,
        /// Dry-run: validate DAG and print shards but don't execute.
        #[arg(long)]
        dry_run: bool,
    },
    /// Resume a previously created session.
    Resume {
        /// Session name to resume.
        name: String,
        #[arg(short = 'd', long, default_value = "dag.db")]
        db: String,
        #[arg(long, default_value = "2")]
        max_parallel: usize,
        #[arg(long)]
        agent: Option<String>,
        #[arg(long)]
        agent_args: Vec<String>,
    },
    /// Print the status of a session (no execution).
    Check {
        /// Session name.
        name: String,
        #[arg(short = 'd', long, default_value = "dag.db")]
        db: String,
    },
    /// Delete a session from the store.
    Delete {
        /// Session name.
        name: String,
        #[arg(short = 'd', long, default_value = "dag.db")]
        db: String,
        /// Skip confirmation prompt.
        #[arg(long)]
        force: bool,
    },
}

impl DagCommand {
    pub fn run(&self) -> Result<()> {
        match &self.sub {
            DagSub::Run { name, db, max_parallel, agent, agent_args, dry_run } => {
                cmd_run(name, db, *max_parallel, agent.as_deref(), agent_args, *dry_run)
            }
            DagSub::Resume { name, db, max_parallel, agent, agent_args } => {
                cmd_resume(name, db, *max_parallel, agent.as_deref(), agent_args)
            }
            DagSub::Check { name, db } => cmd_check(name, db),
            DagSub::Delete { name, db, force } => cmd_delete(name, db, *force),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers: build a smoke-test DAG (mimics the unit-test fixture)
// ---------------------------------------------------------------------------

fn build_dag(name: &str) -> Dag {
    Dag::new(name)
        .task(
            Task::builder("research", "Research phase")
                .priority(Priority::High)
                .build(),
        )
        .task(
            Task::builder("design", "Design the architecture")
                .dep("research")
                .priority(Priority::High)
                .build(),
        )
        .task(
            Task::builder("spec", "Write specification")
                .dep("research")
                .priority(Priority::Medium)
                .build(),
        )
        .task(
            Task::builder("impl", "Implementation")
                .dep("design")
                .dep("spec")
                .priority(Priority::High)
                .build(),
        )
}

fn build_executor(name: &str, store: Arc<SessionStore>) -> Result<Executor> {
    let dag = build_dag(name);
    let sid = store.begin(name, &dag).context("failed to begin session")?;
    Ok(Executor::new(dag, sid, store))
}

// ---------------------------------------------------------------------------
// Subcommand implementations
// ---------------------------------------------------------------------------

fn cmd_run(
    name: &str,
    db: &str,
    max_parallel: usize,
    agent_bin: Option<&str>,
    agent_args: &[String],
    dry_run: bool,
) -> Result<()> {
    let store = Arc::new(SessionStore::open(db)?);
    let mut ex = build_executor(name, store.clone())?;

    println!("DAG  {}  ({} tasks)", name, ex.dag.tasks.len());

    let disp: Box<dyn Dispatcher> = match agent_bin {
        Some(bin) => {
            Box::new(ProcessDispatcher::new(ProcessConfig {
                bin: bin.to_string(),
                args: agent_args.to_vec(),
                ..Default::default()
            }))
        }
        None => {
            println!("  (no --agent specified → noop / dry-run)");
            Box::new(NoopDispatcher::new())
        }
    };

    if dry_run {
        let levels = ex.dag.levels().context("DAG validation failed")?;
        println!("\nShards per level:");
        for (i, lvl) in levels.iter().enumerate() {
            println!("  level {}: {} shard(s)", i, lvl.len());
            for tid in lvl {
                if let Some(shard) = ex.dag.shard(tid) {
                    println!(
                        "    - {}  files=[{}]",
                        shard.task_id,
                        shard.files.join(", ")
                    );
                }
            }
        }
        println!("\n  Dry-run only — no tasks executed.");
        return Ok(());
    }

    // Execute level by level
    let report = ProgressReport::new();
    loop {
        let shards: Vec<Shard> = {
            let batch = ex.next_shards(max_parallel);
            batch.into_iter().filter_map(|t| ex.dag.shard(&t)).collect()
        };
        if shards.is_empty() && ex.is_complete() {
            break;
        }
        for shard in &shards {
            let prompt = build_agent_prompt(shard, "Execute this DAG shard.");
            let manifest = disp.dispatch(shard, &prompt);
            ex.record_result(shard, manifest);
        }
        report.render(&ex.dag, &ex);
    }

    store.finish(ex.session_id)?;
    report.render(&ex.dag, &ex);
    println!("\nSession {} complete.", name);
    Ok(())
}

fn cmd_resume(
    name: &str,
    db: &str,
    max_parallel: usize,
    agent_bin: Option<&str>,
    agent_args: &[String],
) -> Result<()> {
    let store = Arc::new(SessionStore::open(db)?);
    let sid = store
        .find_session(name)?
        .context(format!("session '{}' not found", name))?;

    let mut dag = build_dag(name);
    store.rehydrate(&mut dag, sid)?;

    let mut ex = Executor::new(dag, sid, store.clone());

    let disp: Box<dyn Dispatcher> = match agent_bin {
        Some(bin) => Box::new(ProcessDispatcher::new(ProcessConfig {
            bin: bin.to_string(),
            args: agent_args.to_vec(),
            ..Default::default()
        })),
        None => Box::new(NoopDispatcher::new()),
    };

    let report = ProgressReport::new();
    loop {
        let shards: Vec<Shard> = {
            let batch = ex.next_shards(max_parallel);
            batch.into_iter().filter_map(|t| ex.dag.shard(&t)).collect()
        };
        if shards.is_empty() && ex.is_complete() {
            break;
        }
        for shard in &shards {
            let prompt = build_agent_prompt(shard, "Resume and execute this DAG shard.");
            let manifest = disp.dispatch(shard, &prompt);
            ex.record_result(shard, manifest);
        }
        report.render(&ex.dag, &ex);
    }

    store.finish(ex.session_id)?;
    report.render(&ex.dag, &ex);
    println!("\nSession {} resumed and completed.", name);
    Ok(())
}

fn cmd_check(name: &str, db: &str) -> Result<()> {
    let store = Arc::new(SessionStore::open(db)?);
    let sid = store
        .find_session(name)?
        .context(format!("session '{}' not found", name))?;

    let mut dag = build_dag(name);
    store.rehydrate(&mut dag, sid)?;

    let ex = Executor::new(dag, sid, store.clone());

    let report = ProgressReport::new();
    report.render(&ex.dag, &ex);

    let counts = ex.dag.status_counts();
    println!("\nStatus counts:");
    for (st, n) in &counts {
        println!("  {:?}: {}", st, n);
    }
    Ok(())
}

fn cmd_delete(name: &str, db: &str, force: bool) -> Result<()> {
    if !force {
        eprint!("Delete session '{}' from '{}'? [y/N] ", name, db);
        use std::io::{self, BufRead, Write};
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line)?;
        if line.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return Ok(());
        }
    }

    let store = SessionStore::open(db)?;
    let sid = store
        .find_session(name)?
        .context(format!("session '{}' not found", name))?;
    store.delete(sid)?;
    println!("Session '{}' deleted.", name);
    Ok(())
}
