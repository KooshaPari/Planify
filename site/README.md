# planify.space — landing site

Static landing page for [Planify](https://github.com/KooshaPari/Planify). Lives at `planify.space` (or `planify.kooshapari.com` as fallback).

## Stack

- **Astro 6** — static-site generator with islands architecture
- **Bun** — runtime + package manager (matches sibling Phenotype landings in `phenotype-landing`)
- **Tailwind 4** — utility CSS via `@tailwindcss/vite` plugin
- **Three.js** — 3D hero scene with placeholder keyboard geometry; will swap to `keyboard.glb` once added

## Quick start

```bash
bun install
bun run dev          # http://localhost:4321
bun run build
bun run preview      # preview built output
bun run check        # Astro type-check
```

## Deploy

Vercel — see `vercel.json`. Domain: `planify.space` (or `planify.kooshapari.com`).

### Deploy workflow

1. Push to `main` triggers Vercel preview deploy
2. PR merge to `main` triggers production deploy
3. Vercel project ID is stored in `.vercel/project.json` (gitignored)
4. Environment variables managed via Vercel dashboard, not in repo

### Production vs preview

- **Preview:** `planify-*.vercel.app` — every PR gets one
- **Production:** `planify.space` — gated on `main` branch + Vercel protection rules

## File map

```
site/
├── astro.config.mjs     # Astro config (integrations: tailwind)
├── vercel.json          # Vercel build/deploy config
├── tsconfig.json        # TypeScript config
├── package.json         # Bun + Astro + Tailwind deps
├── data/
│   └── config.json      # single source of truth for the page (site title, copy, links)
├── public/
│   ├── favicon.svg
│   └── keyboard.glb     # TBD — see Asset TODO below
└── src/
    ├── pages/
    │   └── index.astro  # landing page
    ├── components/
    │   └── HeroScene.astro  # 3D canvas (Three.js)
    └── styles/          # (Tailwind handles globals via @tailwindcss/vite)
```

## Asset TODO

- `public/keyboard.glb` — drop the keyboard `.glb` here and the HeroScene component will
  pick it up via GLTFLoader. Until then the placeholder geometry renders.

Tracked as R38 audit bead `e36282cf`; will be addressed in PLAN.md M1-R42.

## Environment variables

None required for build. If `site/data/config.json` needs runtime overrides, use Astro's
`import.meta.env` pattern and configure via Vercel dashboard.

## Common tasks

### Update landing copy

Edit `site/data/config.json` and the corresponding text in `src/pages/index.astro`. The
config file is the single source of truth for site title, tagline, CTA URLs.

### Add a new component

1. Create `src/components/<Name>.astro`
2. Import in `src/pages/index.astro`
3. Add styles via Tailwind utility classes (no per-component CSS file)
4. Test with `bun run dev`

### Update dependencies

```bash
bun update                  # update all
bun add <package>           # add new
bun remove <package>        # remove
```

Bun updates `package.json` and `bun.lockb`. Commit both.

## Troubleshooting

### `bun install` fails

- Check `bun.lockb` matches `package.json` versions; delete and re-install
- Verify Node.js version per `package.json` `engines.node`

### Hero scene not rendering

- Verify `public/keyboard.glb` exists or accept placeholder geometry
- Check browser console for Three.js errors
- Verify Astro client directives (`client:load` for interactive components)

### Vercel build fails

- Check Vercel build logs for Astro / Tailwind errors
- Verify environment variables in Vercel dashboard
- Compare local `bun run build` output to Vercel output

## Related docs

- [`../README.md`](../README.md) — repo overview
- [`../AGENTS.md`](../AGENTS.md) — branching, commit, upstream-sync policy
- [`../CLAUDE.md`](../CLAUDE.md) — root governance pointer
- [`../STATUS.md`](../STATUS.md) — current state + roadmap
- [`../PLAN.md`](../PLAN.md) — Q3-Q4 milestones
- [`../docs/adr/`](../docs/adr/) — architecture decision records
- [`../infra/README.md`](../infra/README.md) — compose-file ops
