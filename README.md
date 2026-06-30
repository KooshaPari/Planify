# Planify

**Phenotype PM Web Frontend** — Consolidated Plane.so fork

Planify is the web-based project management UI for the Phenotype platform, derived from [Plane](https://github.com/makeplane/plane). It powers the AgilePlus dashboard and integrates with the Phenotype ecosystem.

## Status

This repo is currently a stub. See [UPSTREAM.md](./UPSTREAM.md) for seeding instructions.

## Deployment

- **Web**: Vercel (phenotype.space subdomain or direct deployment)
- **Integration**: Embedded in AgilePlus (Rust backend) for unified PM experience

## Architecture

Planify is the web frontend counterpart to AgilePlus (Rust backend PM substrate). Together they provide:
- Dashboard UI (Planify)
- Backend API & storage (AgilePlus)
- Shared domain models (phenotype-domain SDKs)

## Getting Started

1. Seed from upstream: see [UPSTREAM.md](./UPSTREAM.md)
2. Install dependencies (typically Node + Bun/npm)
3. Build & test
4. Deploy to Vercel

## Links

- **AgilePlus**: https://github.com/KooshaPari/AgilePlus
- **phenotype.space**: phenotype platform hub
- **Plane upstream**: https://github.com/makeplane/plane
