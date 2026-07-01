# SRS SvelteKit Rewrite Plan

## Goal

Rewrite the spaced repetition demo as a focused vocabulary learning app that works well at `https://demos.corynorris.me/srs/`, supports real course and word management, and has a reliable review loop.

The existing Rust API and React frontend are preserved in `legacy/` for reference. The new app is a single SvelteKit service at the repository root.

## Product Principles

- Optimize for adding vocabulary quickly.
- Make the review screen fast, keyboard-friendly, and mobile-friendly.
- Store review state durably and keep an append-only review history.
- Keep deployment simple: one web service plus Postgres.
- Use proven scheduling logic rather than inventing an algorithm.
- Prefer server-side form actions for CRUD flows so forms work consistently under the `/srs` base path.

## Chosen Stack

- SvelteKit with TypeScript for the full-stack app.
- SvelteKit adapter-node for Dokploy/Docker deployment.
- PostgreSQL for durable user, course, note, card, and review data.
- Drizzle ORM and drizzle-kit for schema and migrations.
- Better Auth for cookie-backed sessions.
- Zod for form, import, and endpoint validation.
- ts-fsrs for spaced repetition scheduling.
- Vitest for service-level tests.
- Playwright for browser flow tests.

## Deployment Model

The previous deployment had two containers:

- `srs-api`: Rust GraphQL API.
- `srs-web`: React SPA served by nginx.

The rewrite should deploy one container:

- `srs`: SvelteKit Node server on port `3000`.

The parent nginx router should proxy `/srs/` directly to the SvelteKit server without stripping the prefix. SvelteKit will be configured with `BASE_PATH=/srs`, so generated links and assets stay under `/srs`.

Required runtime environment:

```text
DATABASE_URL=postgres://...
BETTER_AUTH_SECRET=...
BETTER_AUTH_URL=https://demos.corynorris.me/srs
BASE_PATH=/srs
PORT=3000
HOST=0.0.0.0
```

## Route Map

```text
/srs/
  Public landing page.

/srs/login
/srs/register
  Auth pages.

/srs/app
  Course dashboard.

/srs/app/courses/new
  Create course.

/srs/app/courses/[courseId]
  Course detail, word table, search, filters, stats.

/srs/app/courses/[courseId]/import
  Paste TSV/CSV import and preview validation.

/srs/app/courses/[courseId]/study
  Due card review session.

/srs/app/settings
  User preferences, default card generation, account controls.

/srs/api/health
  Container health endpoint.
```

## Data Model

### Users and Sessions

Better Auth owns the auth/session tables. The app should expose a normalized `locals.user` with `id`, `email`, and `name`.

### Courses

`course`

- `id`
- `owner_id`
- `title`
- `description`
- `source_language`
- `target_language`
- `is_archived`
- `created_at`
- `updated_at`

Notes:

- `owner_id + title` is unique.
- Archive instead of hard delete in the UI. Hard delete can remain available for explicit destructive actions.

### Lessons

`lesson`

- `id`
- `course_id`
- `title`
- `order_index`
- `created_at`
- `updated_at`

Lessons are optional grouping. The app should allow a course to work without lessons.

### Notes

`note`

- `id`
- `course_id`
- `lesson_id`
- `term`
- `reading`
- `definition`
- `example`
- `example_translation`
- `part_of_speech`
- `notes`
- `tags`
- `created_at`
- `updated_at`

A note is the canonical vocabulary entry. Users edit notes, and cards are derived from notes.

### Cards

`card`

- `id`
- `note_id`
- `kind`
- `prompt`
- `answer`
- `extra`
- `is_suspended`
- `created_at`
- `updated_at`

Initial generated card kinds:

- `term_to_definition`
- `definition_to_term`

Later card kinds:

- `reading_to_definition`
- `example_cloze`
- `listening`

### Review State

`review_state`

- `card_id`
- `state`
- `due_at`
- `stability`
- `difficulty`
- `elapsed_days`
- `scheduled_days`
- `reps`
- `lapses`
- `updated_at`

This table stores the current FSRS state for each card.

### Review Logs

`review_log`

- `id`
- `card_id`
- `rating`
- `reviewed_at`
- `previous_state`
- `next_state`

This table is append-only. It supports stats, debugging, and future analytics.

### Tags

`note_tag`

- `note_id`
- `tag`

The schema also stores `note.tags` as an array for quick display. `note_tag` exists for indexed search/filtering.

## Core User Flows

### First Run

1. User registers.
2. User lands on `/app`.
3. Empty dashboard shows a create-course action.
4. User creates a course.
5. Course detail page prompts for first words.

### Add Words

Single-entry form:

- term
- reading
- definition
- example
- tags
- card types to generate

Bulk-entry form:

```text
term<TAB>definition<TAB>tag1,tag2
```

After submit:

1. Validate entries with Zod.
2. Insert notes.
3. Generate selected cards.
4. Create initial review states due immediately.
5. Return to course detail with added notes visible.

### Search Words

Course detail page should support:

- text search across term, reading, definition, example, and tags.
- filters for due, new, suspended, and difficult.
- inline edit for small changes.

### Study Session

1. Load due cards for the course.
2. Show prompt.
3. Reveal answer.
4. User grades `again`, `hard`, `good`, or `easy`.
5. Use `ts-fsrs` to calculate next review state.
6. Write `review_state` and append `review_log` in one transaction.
7. Continue until the queue is empty or user ends session.

Keyboard controls:

- Space: reveal.
- 1: Again.
- 2: Hard.
- 3: Good.
- 4: Easy.

## Server Architecture

```text
src/lib/server/db/
  schema.ts       Drizzle schema.
  client.ts       Postgres client and typed Drizzle instance.

src/lib/server/repositories/
  courses.ts      Course queries and mutations.
  notes.ts        Note/card queries and mutations.
  reviews.ts      Due queue and review persistence.

src/lib/server/services/
  scheduler.ts    ts-fsrs adapter and rating conversion.
  import.ts       TSV/CSV parsing and validation.
  permissions.ts  Ownership checks.

src/lib/validation/
  course.ts
  note.ts
  review.ts
```

Repository functions should be boring and explicit. Business rules such as ownership, card generation, and scheduling should live in services.

## SvelteKit Patterns

- Use `+page.server.ts` load functions for protected data.
- Use form actions for create/update/delete flows.
- Use endpoint routes only for health checks or progressive client interactions that genuinely need fetch.
- Keep auth checks server-side.
- Use `$app/paths` for base-path-safe links in Svelte components.

## Milestones

### Milestone 1: Buildable App and Auth

- Install dependencies and commit `pnpm-lock.yaml`.
- Wire Better Auth to Postgres.
- Populate `event.locals.user`.
- Implement login/register/signout.
- Add `/app` route protection.
- Verify local build and Docker build.

### Milestone 2: Courses

- Generate first Drizzle migration.
- Implement course repository.
- Add create/edit/archive course actions.
- Add dashboard course list and empty states.
- Add tests for course ownership.

### Milestone 3: Notes and Cards

- Implement note repository.
- Implement card generation from notes.
- Add course detail page.
- Add single word create/edit/delete.
- Add search/filter.
- Add due/new/suspended counts.

### Milestone 4: Bulk Import

- Add TSV paste import.
- Add CSV file import.
- Add preview page with row-level validation errors.
- Add duplicate detection.
- Add import tests.

### Milestone 5: Review Loop

- Implement `ts-fsrs` adapter.
- Add due queue repository.
- Add study page and grading actions.
- Record review logs transactionally.
- Add Playwright test for a full review session.

### Milestone 6: Polish and Deployment

- Mobile review layout.
- Keyboard shortcuts.
- Course stats.
- Export notes and review history.
- Production Docker build.
- Parent compose/nginx integration.

## Migration From Legacy

The legacy app did not have usable user content beyond courses in the deployed UI. If existing production course data matters, migrate:

1. Keep old Postgres database untouched.
2. Create a one-time script that reads old `user` and `course` rows.
3. Map users to Better Auth users by email.
4. Insert courses into the new schema.
5. Do not migrate old cards until the old card JSON shapes are audited.

If production data is disposable, start with a fresh schema in the same Postgres instance and keep the old tables until the new app has shipped.

## Open Decisions

- Whether registration should be open to the public or restricted by invite code.
- Whether courses should ever be public/shareable.
- Whether generated cards should be configurable per course or per note.
- Whether examples/audio should be plain fields or separate media tables.
- Whether to support offline review later.

## Immediate Next Steps

1. Install dependencies and lock versions.
2. Fix any SvelteKit scaffold compile errors.
3. Wire Better Auth.
4. Generate the first Drizzle migration.
5. Implement course CRUD.
