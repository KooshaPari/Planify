# Secrets Rotation Guide

This document lists all secrets that were potentially exposed and must be rotated.

## Potentially Exposed Secrets

### 1. Django SECRET_KEY

- **Value**: `ede34d5af6907978dcdeddb57de0f2cb023cf37e49acdc485cbd573b76fd9f88`
- **Location**: `DATABASE_URL` environment variable (local `.env` only)
- **Risk**: HIGH — Used for Django session signing, CSRF tokens, password reset tokens
- **Rotation**:
  ```bash
  # Generate new key
  openssl rand -hex 64
  # Update in .env and secrets manager
  # Then restart all Plane services
  ```

### 2. PostgreSQL Password (`agileplus-dev`)

- **Value**: `agileplus-dev`
- **Location**: `postgresql://agileplus:agileplus-dev@localhost:5432/plane` in `DATABASE_URL`
- **Risk**: HIGH — Direct database access if PostgreSQL port is exposed
- **Rotation**:
  ```bash
  # In PostgreSQL:
  ALTER USER agileplus WITH PASSWORD 'new-secure-password';
  # Then update DATABASE_URL in .env and secrets manager
  ```

### 3. Redis (low risk)

- **Value**: `redis://localhost:6379`
- **Risk**: LOW — localhost only by default

### 4. AWS Credentials (if configured)

- **Risk**: HIGH — if `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` were set
- **Rotation**: Via AWS IAM console → Security credentials → Create new access key

## How to Rotate

### Option 1: Use the rotation script

```bash
cd apps/AgilePlus/.agileplus/plane
./scripts/rotate-secrets.sh
```

### Option 2: Manual rotation

1. Generate new secrets:

   ```bash
   # New Django SECRET_KEY
   openssl rand -hex 64

   # New PostgreSQL password
   openssl rand -base64 32 | tr -dc 'A-Za-z0-9' | head -c 24
   ```

2. Update database:

   ```bash
   psql postgresql://agileplus:NEW_PASSWORD@localhost:5432/plane -c \
     "ALTER USER agileplus WITH PASSWORD 'NEW_PASSWORD';"
   ```

3. Update your `.env` file:

   ```
   SECRET_KEY=<new-openssl-rand-hex-64-output>
   DATABASE_URL=postgresql://agileplus:NEW_PASSWORD@localhost:5432/plane
   ```

4. Update secrets manager (Doppler, Vault, AWS Secrets Manager, etc.)

5. Restart all Plane services

### Option 3: Doppler

```bash
# Update secrets in Doppler dashboard, then:
doppler run -- bun dev
```

## Prevention

- **Never commit `.env` files** — `.gitignore` already excludes them
- **Use secrets manager** — Doppler, Vault, AWS Secrets Manager, or 1Password CLI
- **Use `.env.example` as template** — Copy to `.env` and fill in real values
- **Run `ggshield secret scan repo .`** before pushing to verify no secrets committed

## Git History Check

To verify no secrets are in git history:

```bash
ggshield secret scan repo .
```

Or with git log:

```bash
git log --all -p -S "agileplus-dev" | grep -i password
git log --all -p -S "ede34d5af6907978" | grep SECRET_KEY
```

## Files Changed for Secrets Management

| File                                | Purpose                                          |
| ----------------------------------- | ------------------------------------------------ |
| `.gitignore`                        | Explicitly excludes `.env`, `.venv/`, `scripts/` |
| `.env.example`                      | Template for local development                   |
| `doppler.yaml`                      | Doppler configuration for CI/CD                  |
| `scripts/rotate-secrets.sh`         | Automated rotation script                        |
| `apps/api/plane/settings/common.py` | Reads from env vars (no hardcoding)              |
