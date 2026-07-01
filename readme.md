# Spaced Repetition

SvelteKit rewrite scaffold for the `/srs` demo app.

The previous Rust API and React frontend have been moved to `legacy/` while the rewrite is built. See [docs/design-plan.md](docs/design-plan.md) for the target architecture and implementation milestones.

## Development

```bash
pnpm install
pnpm dev
```

For production-like local routing under `/srs`:

```bash
BASE_PATH=/srs pnpm dev
```

## Deployment

The intended production deployment is a single SvelteKit Node container on port `3000`, backed by Postgres.

Required environment:

```text
DATABASE_URL=postgres://...
BETTER_AUTH_SECRET=...
BETTER_AUTH_URL=https://demos.corynorris.me/srs
BASE_PATH=/srs
PORT=3000
HOST=0.0.0.0
```
