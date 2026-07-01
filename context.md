# Codebase Context: Spaced Repetition App

## 1. What the App Does

A **spaced repetition learning platform** — users create accounts, organize content into **courses → lessons → cards**, and study using spaced repetition. Cards are typed (via `card_type` table with JSON schema) supporting flashcard-style learning with JSON content blobs.

Current state: **early-stage MVP**. The backend has full user auth (register/login/JWT), course CRUD, and database schema for cards/lessons/card_types. The frontend is a skeleton: only login/register/dashboard pages exist. No study/review logic is implemented yet. Two hooks (`useStudySession`, `useSyncStore`) are fully commented out as placeholders.

## 2. Tech Stack and Key Dependencies

### Backend (Rust)
| Layer | Technology |
|---|---|
| Runtime | `tokio` (full features) |
| Web framework | `axum 0.7.9` |
| GraphQL | `async-graphql 7.0.13` + `async-graphql-axum` |
| Database | PostgreSQL 16 via `sqlx 0.8.2` (compile-time checked queries) |
| Auth | `argon2` password hashing, `jwt` + `hmac` + `sha2` for JWTs |
| Config | `clap 4.5` (CLI args + env vars), `dotenvy` |
| Logging | `tracing`, `tracing-subscriber`, `env_logger` |
| Rate limiting | `governor 0.8.0` (dep added, but **not wired in**) |
| Testing | `mockall 0.12.1` (dep added, but **no mocks used**) |

### Frontend (React/TypeScript)
| Layer | Technology |
|---|---|
| Bundler | Vite 6 |
| UI Framework | React 18.3 |
| Routing | `@tanstack/react-router` |
| State management | `zustand` with `persist` middleware |
| Server state | `@tanstack/react-query` |
| Forms | `react-hook-form` + `zod` |
| Styling | Tailwind CSS 3.4 |
| Icons | `lucide-react` |
| Package manager | `pnpm` |

### Infrastructure
- **Docker Compose** for local dev (PostgreSQL 16 Alpine)
- **Dokku** for deployment (custom deploy script using `git subtree push`)
- API and Web are separate Dokku apps: `srs-api` (port 80→5000) and `srs-web`
- Dockerfiles: Multi-stage Rust build → Debian slim; Multi-stage Node build → Nginx Alpine

## 3. File Structure Overview

```
spaced-repitition/
├── readme.md                          # EMPTY
├── app.json                           # Dokku app metadata ("spaced-repetition-api")
├── spaced-repetition.code-workspace   # VS Code multi-root: api/ + web/
├── scripts/
│   ├── deploy.sh                      # Dokku deploy via git subtree push
│   └── dev.sh                         # Local dev: docker compose up db, cargo run &, pnpm dev
│
├── api/                               # Rust backend
│   ├── Cargo.toml                     # Dependencies
│   ├── Cargo.lock
│   ├── Dockerfile                     # Multi-stage: rust:latest → debian:bookworm-slim
│   ├── docker-compose.yaml            # PostgreSQL 16, port 5432
│   ├── .env.sample                    # DATABASE_URL, HMAC_KEY, RUST_LOG
│   ├── .gitignore                     # Only /target
│   ├── readme.md                      # Quickstart + Dokku deploy notes
│   ├── scripts/setup.sh               # Interactive dev setup (install sqlx-cli, run migrations)
│   ├── migrations/
│   │   ├── 20241224180811_setup.sql   # uuid-ossp, trigger_updated_at, case_insensitive collation
│   │   ├── 20241224180822_create_users.sql  # role enum, "user" table
│   │   └── 20241225214442_create_cards.sql  # card_type, course, lesson, card tables + materialized view
│   └── src/
│       ├── main.rs                    # Entry: dotenv, Config::parse, db pool, migrate, start_server
│       ├── lib.rs                     # Module declarations
│       ├── config.rs                  # Config struct (clap Parser with env support)
│       ├── domain/
│       │   ├── auth.rs               # AuthKey (HMAC-SHA384), AuthUser, PasswordManager, JWT sign/verify
│       │   ├── errors.rs             # DomainError enum (12 variants)
│       │   ├── models/
│       │   │   ├── card.rs           # EMPTY FILE
│       │   │   ├── course.rs         # Course entity + CourseSummary + validation
│       │   │   ├── lesson.rs         # Lesson entity + validation
│       │   │   └── user.rs           # User entity, Role enum, validation, input DTOs
│       │   ├── policies/
│       │   │   ├── user_policy.rs    # UserPolicy: can_view, can_update, can_delete, can_change_role...
│       │   │   └── course_policy.rs  # CoursePolicy: can_view, can_modify, can_list_user_courses
│       │   ├── repositories/
│       │   │   ├── user_repository.rs    # UserRepository: CRUD + verify_credentials
│       │   │   └── course_repository.rs  # CourseRepository: CRUD + search + summary queries
│       │   └── services/
│       │       ├── user_service.rs   # UserService: register, login, get_user, change_password, etc.
│       │       └── course_service.rs # CourseService: create, update, publish, delete, search
│       └── application/
│           ├── container.rs          # ServiceContainer, ApiContext (DI container)
│           ├── http.rs               # Axum router, /api/graphql endpoint, GraphiQL, auth extraction
│           └── graphql/
│               ├── schema.rs         # Schema builder (QueryRoot, MutationRoot, EmptySubscription)
│               ├── scalars.rs        # Timestamptz scalar (RFC3339)
│               ├── errors.rs         # GraphQLError wrapper, ErrorCode enum, validation error formatting
│               ├── guards/
│               │   └── role_guard.rs # RoleGuard (Role::User/Admin), auto-includes Admin
│               ├── middleware/
│               │   └── error_interceptor.rs  # ErrorInterceptor extension, SchemaErrorHandler trait
│               ├── resolvers/
│               │   ├── user_queries.rs      # QueryRoot.users: me(), user(id)
│               │   ├── user_mutations.rs    # MutationRoot.users: register, login, update_profile
│               │   └── admin_mutations.rs   # Admin-level: update_user_profile, change_role, delete_user
│               └── types/
│                   ├── user.rs       # UserObject, CreateUserInput, LoginInput, AuthPayload, etc.
│                   ├── course.rs     # CourseObject, CourseSummaryObject, CreateCourseInput, etc.
│                   ├── lesson.rs     # LessonObject, CreateLessonInput, UpdateLessonInput
│                   ├── card.rs       # Card (GraphQL object), CardRow, CreateCardInput, UpdateCardInput
│                   ├── card_type.rs  # CardType, CardTypeRow, CreateCardTypeInput
│                   └── cursor.rs     # CursorInput, CursorResult<T> (pagination — UNUSED)
│
└── web/                               # React frontend
    ├── package.json                   # pnpm, React 18, TanStack Router/Query, Zustand, Zod
    ├── pnpm-lock.yaml
    ├── vite.config.ts                 # @ alias, proxy /api → localhost:5000
    ├── tsconfig.json / tsconfig.app.json / tsconfig.node.json
    ├── tailwind.config.js
    ├── postcss.config.js
    ├── eslint.config.js
    ├── index.html                     # Default Vite template title ("Vite + React + TS")
    ├── Dockerfile                     # Multi-stage: node:20-alpine → nginx:alpine
    ├── nginx.conf                     # SPA fallback (try_files $uri /index.html)
    ├── .env.sample                    # VITE_API_URL=http://localhost:5000
    └── src/
        ├── main.tsx                   # React root render
        ├── App.tsx                    # QueryClientProvider + RouterProvider with auth context
        ├── router.tsx                 # rootRoute, route tree (login, register, dashboard)
        ├── index.css                  # Tailwind directives only
        ├── vite-env.d.ts              # Vite client types
        ├── utils/env.ts               # isDev, isProd helpers (process.env.NODE_ENV)
        ├── features/
        │   ├── auth/
        │   │   ├── types.ts           # User, LoginCredentials, RegisterCredentials interfaces
        │   │   ├── store.ts           # Zustand store (persist to localStorage, devtools in dev)
        │   │   ├── routes.ts          # /login and /register routes with publicOnlyRoute guard
        │   │   ├── hooks/
        │   │   │   ├── useAuthBase.ts # Core auth logic: useQuery for user, useMutation for login/register
        │   │   │   └── useAuth.ts     # Wraps useAuthBase with navigation (login→redirect to /)
        │   │   ├── components/
        │   │   │   ├── LoginForm.tsx  # react-hook-form + zod, calls login mutation
        │   │   │   └── RegistrationForm.tsx  # Same pattern for register
        │   │   └── pages/
        │   │       ├── LoginPage.tsx  # Centers LoginForm with link to register
        │   │       └── RegisterPage.tsx # Centers RegisterForm with link to login
        │   ├── dashboard/
        │   │   ├── routes.ts          # / route with protectRoute guard
        │   │   └── pages/
        │   │       └── DashboardPage.tsx  # Shows user info + logout button + JSON dump
        │   └── routes/
        │       └── protect.ts         # protectRoute() and publicOnlyRoute() redirect guards
        └── shared/
            └── hooks/
                ├── useStudySession.ts # EMPTY (commented-out placeholder)
                └── useSyncStore.ts    # EMPTY (commented-out placeholder)
```

## 4. How It's Built/Deployed

### Local Development
```bash
# In api/
docker compose up -d db          # Start PostgreSQL
sqlx migrate run                 # Run migrations
cargo run                        # Starts on 0.0.0.0:8080

# In web/
pnpm dev                         # Vite dev server, proxies /api → localhost:5000
```

Alternatively, `scripts/dev.sh` starts both (API in background, frontend in foreground).

### Production Deployment (Dokku)
```bash
# First deploy: scripts/deploy.sh creates Dokku apps and pushes via git subtree
# API: git subtree push --prefix api dokku-api main
# Web: git subtree push --prefix web dokku-web main
```
- API: Dockerfile builds Rust release binary, runs on port 5000, Dokku maps port 80→5000
- Web: Dockerfile builds Vite production bundle, served by Nginx on port 80

### Build System
- **API**: `cargo build --release` (no special features), SQLx migrations run at startup via `sqlx::migrate!().run()`
- **Web**: `tsc -b && vite build` → static files in `dist/`

## 5. Obvious Issues and Outdated Patterns

### Critical Issues
1. **Domain card model is empty** — `api/src/domain/models/card.rs` is a zero-byte file. The DB schema and GraphQL types exist, but the domain model (which should be the source of truth per the DDD-ish architecture) is missing. This means no repository or service exists for cards, and the GraphQL `Card` type uses a raw `CardRow` instead.

2. **Frontend API mismatch with backend GraphQL** — The frontend makes REST calls to `/api/users/login`, `/api/users`, `/api/user` but the backend only exposes a **GraphQL** endpoint at `/api/graphql`. The mutation/query names don't match at all. The frontend auth will always fail against the actual backend.

3. **Port mismatch** — Dockerfile exposes port 5000, `cargo run` default is 8080 (from Config), but `scripts/dev.sh` starts the API without overriding the port. Vite proxies to port 5000. Result: nothing works out of the box.

4. **`publish_course` unconditionally publishes** — `course_service.rs::publish_course()` always calls `set_published(course_id, true)`. There's no way to **unpublish** a course via the service layer, even though the repository supports it.

5. **Admin mutations not wired to schema** — `admin_mutations.rs` defines `AdminMutation` but it's never included in the schema. The `MutationRoot` only exposes `users → UserMutation`. Admin operations (update_user_profile, change_role, delete_user) are completely inaccessible via GraphQL.

### Architecture/Design Issues
6. **Course and Lesson types defined but never used in GraphQL schema** — `CourseObject`, `CourseSummaryObject`, `LessonObject`, `Card`, `CardType` are all defined as GraphQL types but never registered on the query/mutation roots. The schema only serves user queries/mutations.

7. **Unused dependencies** — `governor` (rate limiting) and `mockall` (mocking) are in `Cargo.toml` but have zero usage in the codebase.

8. **Rust `latest` base images** — `Dockerfile` uses `rust:latest` and `debian:bookworm-slim`. The `latest` tag is non-deterministic; should pin to a specific version.

9. **No health check endpoint** — No `/health` or readiness probe endpoint for container orchestration.

10. **Dev script has `.env.example` reference** — `api/scripts/setup.sh` references `cp .env.example .env` but the actual file is named `.env.sample`.

11. **`docs.rs`-style doc comment in migration** — `api/migrations/20241224180811_setup.sql` has a long style commentary that says "lowercase SQL" but the migration file uses a mix of lower and UPPER case inconsistently.

12. **`time` crate version is old** — `time = "0.3.37"`. Latest is `0.3.x` but 0.3.37 is from mid-2024. Not critical, but worth bumping.

### Frontend Issues
13. **`title` tag is still "Vite + React + TS"** — `index.html` has the default Vite scaffold title.

14. **`dotenv` package imported but unused in consumer code** — `dotenv` is in web/package.json but only `vite.config.ts` calls `config()` from it. The `process.env` usage in `env.ts` relies on Vite's built-in env handling, making `dotenv` redundant.

15. **Duplicate `vite-env.d.ts`** — exists at both `web/src/vite-env.d.ts` and `web/vite-env.d.ts`. The one at `web/vite-env.d.ts` has extra `ImportMetaEnv` typings for `VITE_API_URL`, while `web/src/vite-env.d.ts` is minimal.

16. **User type mismatch** — Frontend `User` type includes `token` field (used for local persistence), but the backend `AuthPayload` returns `token` at the top level, not nested inside `user`. The frontend treats `token` as part of the user object throughout the auth flow.

17. **`useAuthBase` makes REST calls** — fetches to `/api/user`, `/api/users/login`, `/api/users` — none of these exist on the GraphQL-only backend.

### Security Considerations
18. **No CORS configuration** — Neither `tower-http::cors` nor `axum` CORS layer is configured. This will block browser requests from the web frontend in production.

19. **No input sanitization** — Email validation is just `contains('@') && contains('.')`. No proper email validation crate is used (the code has a TODO comment acknowledging this).

20. **JWT stored in localStorage** — The Zustand auth store persists to localStorage with `name: "auth-storage"`. This is vulnerable to XSS token theft. HttpOnly cookies would be more secure.

21. **`CryptoError` variant accepts raw strings** — `DomainError::CryptographyError(String)` could leak internal crypto details to error messages if not carefully handled.

### Missing Features (intentional, not bugs)
- No card/lesson CRUD implemented (schema exists, resolvers don't)
- No study/review logic (commented-out hooks exist as placeholders)
- No course queries/mutations registered in GraphQL schema (types exist)
- No pagination implemented (CursorResult exists but unused)
- No DataLoader for N+1 queries (referenced in Card.card_type resolver as `todo!()`)
- No tests for services or repositories (only unit tests on policies and validation)
- No refresh token mechanism (JWTs are valid for 2 weeks with no revocation)

## Start Here
For understanding the backend, open `api/src/main.rs` (entry point), then `api/src/application/http.rs` (server setup and GraphQL wiring).

For understanding the frontend, open `web/src/App.tsx` (app shell and auth context) and `web/src/features/auth/hooks/useAuthBase.ts` (core auth logic that exposes the REST/GraphQL mismatch).
