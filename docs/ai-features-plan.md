# SRS AI Features — Architecture & Implementation Plan

## Design Principles

1. **Generic app, selectable language profiles** — the core SRS model remains language-agnostic; course creation lets users choose a target language/profile, and Japanese unlocks richer kanji/hiragana/furigana display support
2. **Card types as a first-class system** — v1 supports recognition, recall, reading recognition, sentence ordering, and cloze; listening, multiple-choice, and typing are post-v1
3. **Lesson types for organizing study modes** — vocabulary, sentence practice, conversation
4. **AI as a content pipeline** — LLM generates structured data that flows into the existing note/card/review system
5. **Preview before persistence** — AI output is validated and shown to the user before creating courses, lessons, notes, or cards
6. **Local-first LLM deployment** — use the proven OpenAI-compatible utilities client against llama.cpp, with optional premium fallback later

---

## Implementation Status — 2026-07-01 (evening)

This plan has been implemented through the first AI/Japanese v1 slice. The app now has the language-profile schema, Japanese display utilities, v1 card-kind builders, AI preview endpoints, confirmed persistence actions, in-course practice generation, scoped discussion, environment documentation, route hardening, interactive multi-card-type review, and focused test coverage.

### Completed

- Added schema and migration support for:
  - `note.furigana`
  - `lesson.type`
  - `lesson.metadata`
  - `course.language_profile`
  - `course.display_options`
- Added validation and normalization for language profiles, display options, notes, and courses.
- Added Japanese utilities:
  - safe bracket-markup furigana parsing
  - kanji detection
  - Japanese answer normalization
  - Japanese display shaping
- Added centralized card-kind definitions and builders for v1:
  - `recognition`
  - `recall`
  - `reading_recognition`
  - `sentence_order`
  - `sentence_cloze`
- Documented post-v1 card kinds:
  - `typing`
  - `multiple_choice`
  - `listening`
- Ported the lightweight OpenAI-compatible LLM client from `/home/cory/Code/dokploy-apps/utilities/server/llm.ts`.
- Added AI helpers for JSON extraction, Zod validation summaries, and in-memory user/scope rate limiting.
- Added AI endpoints:
  - `GET /api/ai/status`
  - `POST /api/ai/generate-course`
  - `POST /api/ai/generate-practice`
  - `POST /api/ai/discuss-content`
- Kept AI endpoints preview-only. Confirmed persistence happens through authenticated SvelteKit page actions.
- Added generated-course persistence that creates courses, lessons, notes, cards, and review-state rows transactionally.
- Added generated-practice persistence for sentence-practice and conversation lessons.
- Updated course creation, note creation, course detail/settings, and review rendering for Japanese fields and display options.
- Hardened AI routes:
  - invalid JSON returns `400`
  - unauthenticated requests return `401`
  - missing or unowned courses return `404`
  - rate limiting happens after request validation and ownership checks
- Added environment variables to `.env.example` and `docker-compose.yml`:
  - `LLM_BASE_URL`
  - `LLM_MODEL`
  - `LLM_API_KEY`
  - `LLM_TIMEOUT_MS`
  - `AI_RATE_LIMIT_PER_USER`
  - `AI_RATE_LIMIT_WINDOW_MS`

### Verified

- `pnpm test` passes with 9 files and 39 tests.
- `pnpm run check` passes with 0 errors and 0 warnings.
- `pnpm run build` passes.
- Workspace connectivity to `http://10.88.111.16:8080/v1/models` returns HTTP 200.
- One-off Docker Compose `srs` container can reach the LLM endpoint; `qwen3.5-4b` reports `loaded`.

### Not Yet Done

- No Playwright UI tests exist yet. The package has a `test:e2e` script, but there is no Playwright config or e2e test directory.
- No dedicated AI tools route exists yet; AI course generation lives on course creation, and in-course practice/discussion lives on the course detail page.
- AI UI polish is still basic:
  - loading states exist, but no skeletons
  - errors are shown inline, but no route-level error boundaries
  - no streaming responses
- Observability is minimal. The app logs failures, but does not yet log model latency, retry attempts, validation-failure counts, or rate-limit denials in a structured way.
- Added interactive review behavior for all v1 card kinds:
  - `sentence_order`: clickable word blocks that rearrange, submit to grade, feedback, auto-rate through `rate` action.
  - `sentence_cloze`: sentence with blank, text input, compare against expected answer (uses `normalizeJapaneseAnswer()` for Japanese), feedback, auto-rate through `rate` action.
  - `recognition`, `recall`, `reading_recognition`: preserved with existing reveal/rating flow.
- Added `src/lib/cards/grading.ts` — answer-grading helpers (`gradeSentenceOrder`, `gradeSentenceCloze`, `isAutoGradedCardKind`, `shuffleIndices`, `splitClozeSentence`).
- Added `src/lib/cards/grading.test.ts` — 17 focused unit tests for grading helpers.
- No duplicate-term cleanup or preview warnings are implemented yet.
- No premium/local LLM fallback chain is implemented.

### Recommended Next Phase

Add Playwright config and e2e tests for Japanese course creation, display toggles, AI preview flows, and multi-card review behavior. Also add loading skeletons, route-level error boundaries, and basic observability. Streaming AI responses and premium/local LLM fallback remain post-v1 polish items.

---

## Part A: Schema & Data Model Changes

### A1. Add `furigana` to notes

```sql
ALTER TABLE note ADD COLUMN furigana TEXT;
```

The `note` table becomes the canonical entry for all 4 Japanese dimensions:

| DB Column    | Japanese Meaning              | Generic Meaning     |
| ------------ | ----------------------------- | ------------------- |
| `term`       | kanji form (食べる)           | The target word     |
| `reading`    | hiragana pronunciation (たべる) | Pronunciation       |
| `furigana`   | furigana annotation           | Ruby/furigana text  |
| `definition` | English meaning (to eat)      | Translation         |

Store `furigana` as plain annotated text, not HTML, so it remains safe to render and easy to validate. Recommended format:

```text
食[た]べる
今日[きょう]は日本語[にほんご]を勉強[べんきょう]します
```

The Japanese adapter is responsible for parsing this into `<ruby>` markup in Svelte. If `furigana` is missing, the UI falls back to `term` plus `reading`.

### A2. Add `type` and `metadata` to lessons

The `lesson` table already exists but is barely wired up. We add:

```sql
ALTER TABLE lesson ADD COLUMN type TEXT NOT NULL DEFAULT 'vocabulary';
ALTER TABLE lesson ADD COLUMN metadata JSONB NOT NULL DEFAULT '{}';
```

**Lesson types:**

- `vocabulary` — standard vocab list (cards: `recognition` + `recall`)
- `sentence_practice` — practice sentences (cards: `sentence_order` + `sentence_cloze`)
- `conversation` — dialogue practice (v1 cards: `sentence_order` + `sentence_cloze`; richer multiple-choice can follow post-v1)
- `ai_generated` — reserved only if we need a generic bucket; prefer `metadata.generatedBy = "ai"` on one of the concrete types above

`metadata` holds lesson-specific data, e.g., for conversations:

```json
{
  "dialogue": [
    { "speaker": "A", "japanese": "すみません、メニューを見せてください", "english": "Excuse me, can I see the menu?" },
    { "speaker": "B", "japanese": "はい、どうぞ", "english": "Yes, here you go." }
  ],
  "setting": "At a restaurant"
}
```

### A3. Add `display_options` and `language_profile` to courses

```sql
ALTER TABLE course ADD COLUMN display_options JSONB NOT NULL DEFAULT '{}';
ALTER TABLE course ADD COLUMN language_profile TEXT;
```

`language_profile`: `null`/`'generic'` (default), `'japanese'`, or future languages.

Generic default `display_options`:

```json
{
  "showTerm": true,
  "showReading": true,
  "showDefinition": true,
  "cardTypes": ["recognition", "recall"],
  "activeCardTypes": ["recognition", "recall"]
}
```

Japanese default `display_options`:

```json
{
  "showKanji": true,
  "showHiragana": false,
  "showFurigana": false,
  "showEnglish": true,
  "cardTypes": ["recognition", "recall"],
  "activeCardTypes": ["recognition", "recall"]
}
```

The SRS app currently has only `sourceLanguage` and `targetLanguage`. `language_profile` should be selected explicitly at course creation, derived when possible for imported/legacy courses (`targetLanguage` equals `ja`, `jpn`, `Japanese`, or `日本語` -> `japanese`), and editable on the settings page.

Settings UI should render controls by profile:

- Generic: show/hide term, reading, definition, examples, enabled card types.
- Japanese: show/hide kanji, hiragana, furigana, English, examples, enabled card types.
- Hide Japanese-only labels and furigana controls for generic courses, but keep the underlying `furigana` column harmlessly nullable.

### A4. Expanded card kinds

No schema change needed — uses existing `kind` (text) + `extra` (JSONB) columns on the `card` table.

| Kind                  | Prompt                            | Answer              | `extra` JSON                                                                              |
| --------------------- | --------------------------------- | ------------------- | ----------------------------------------------------------------------------------------- |
| `recognition`         | Show term                         | Definition          | `{}`                                                                                      |
| `recall`              | Show definition                   | Term                | `{}`                                                                                      |
| `reading_recognition` | Show kanji                        | Reading (hiragana)  | `{}`                                                                                      |
| `sentence_order`      | Show translation + shuffled blocks| Correct order       | `{"word_blocks":["私","は","食べる"],"correct_order":[0,1,2],"translation":"I eat"}`       |
| `sentence_cloze`      | Show sentence with blank          | Missing word        | `{"sentence":"毎日___を食べる","blank_position":1}`                                        |

Post-v1 card kinds are documented but not required for the first AI/Japanese release: `typing`, `multiple_choice`, and `listening`.

Card `kind` remains text for flexibility, but the app should centralize valid kinds in `src/lib/cards/kinds.ts` so validation, generation, and review rendering do not drift.

---

## Part B: Language Profiles & Japanese Display Support

Courses should not be Japanese-only. The default course experience stays generic:

- `term` = prompt/front-side learning item
- `reading` = optional pronunciation/transliteration
- `definition` = answer/translation
- `example` and `exampleTranslation` = optional context

When a user creates or edits a course, they can select a `language_profile`. `generic` is the default. `japanese` is the first rich profile and enables Japanese-specific labels, fields, card kinds, and review display controls.

**New file:** `src/lib/language/japanese.ts`

The DB schema stays generic. TypeScript adapter classes provide language-specific field accessors for the UI layer only.

```typescript
// Generic note row from DB
interface NoteRow {
  term: string
  reading: string | null
  furigana: string | null
  definition: string
  // ... other fields
}

// Japanese-specific accessor (the "concrete subclass")
class JapaneseNote {
  constructor(private note: NoteRow) {}

  get kanji(): string    { return this.note.term }
  get hiragana(): string { return this.note.reading ?? '' }
  get furigana(): string { return this.note.furigana ?? '' }
  get english(): string  { return this.note.definition }

  // Whether to display each field based on course display options
  visibleFields(options: DisplayOptions): string[] {
    const fields: string[] = []
    if (options.showKanji)    fields.push('kanji')
    if (options.showHiragana) fields.push('hiragana')
    if (options.showFurigana) fields.push('furigana')
    if (options.showEnglish)  fields.push('english')
    return fields
  }
}
```

**New file:** `src/lib/language/registry.ts`

```typescript
type LanguageProfile = 'japanese' | 'generic'

function getNoteAdapter(note: NoteRow, profile: LanguageProfile | null) {
  switch (profile) {
    case 'japanese': return new JapaneseNote(note)
    default:         return new GenericNote(note)
  }
}
```

Adding Korean, Chinese, etc. means adding another adapter class — zero schema changes.

### B1. Course Creation Language Selection

The new/edit course flow should expose language choices without turning the app into a Japanese-only product:

| UI Field | Generic Course | Japanese Course |
| -------- | -------------- | --------------- |
| Target language | Free text or select | `Japanese` selectable |
| `language_profile` | `generic` | `japanese` |
| Note field labels | Term, Reading, Definition | Kanji/Kana term, Hiragana reading, Furigana, English |
| Default display options | Show term + definition | Show kanji + English, optionally hiragana/furigana |
| Extra generated cards | Recognition/recall | Recognition/recall + reading recognition when useful |

Course creation behavior:

- Add a language selector with `Generic` and `Japanese` first; future profiles can be added without migrations.
- Selecting `Japanese` sets `targetLanguage = "Japanese"` and `language_profile = "japanese"`.
- Selecting `Generic` leaves Japanese-only display controls hidden.
- The settings page can later change the profile, but warn if switching away from Japanese will hide furigana-specific UI rather than delete stored data.

### B2. Japanese Utilities

The Japanese adapter should include a few small utilities instead of spreading Japanese-specific logic across routes:

- `parseFuriganaMarkup(text)` -> token list for safe `<ruby>` rendering.
- `containsKanji(text)` -> decide whether `reading_recognition` is useful.
- `normalizeJapaneseAnswer(text)` -> trim whitespace, normalize full-width/half-width where practical, and compare kana answers.
- `buildJapaneseDisplay(note, options)` -> one shape consumed by note list, card front/back, and preview screens.

Romaji support should be treated as optional polish. If added, use a library or a tiny explicit kana mapping rather than asking the LLM to grade typed answers.

---

## Part C: LLM Integration

### C1. LLM Client — `src/lib/server/services/llm.ts`

Port the proven client from `/home/cory/Code/dokploy-apps/utilities/server/llm.ts`:

- `callLlm(config, request)` — single call with timeout + abort controller
- `callLlmWithRetry(config, request, options)` — retry with exponential backoff
- Types: `LlmMessage`, `LlmRequest`, `LlmResponse`, `LlmConfig`, `LlmError`
- Strips trailing `/v1` from baseUrl so it doesn't double up
- Handles reasoning models that put output in `reasoning_content` instead of `content`

**Env vars** (added to `.env.example` and `docker-compose.yml`):

```
LLM_BASE_URL=http://10.88.111.16:8080
LLM_MODEL=qwen3.5-4b
LLM_API_KEY=...
LLM_TIMEOUT_MS=60000
AI_RATE_LIMIT_PER_USER=5
AI_RATE_LIMIT_WINDOW_MS=60000
```

Add a small `loadLlmConfig()` beside the client, following `utilities/server/premium-llm.ts`, so routes do not read `process.env` directly. For v1, keep a single local LLM config. The premium/local fallback chain can be added later if this app needs user-specific quotas.

### C1.1. Structured Output Helpers

LLM JSON must be treated as untrusted input:

- `extractJsonObject(content)` strips markdown fences and returns the first complete JSON object.
- Every AI service validates with Zod before returning data to routes.
- Validation errors include a compact path summary for logs, but user-facing errors stay generic.
- Do not persist generated data in the same request that calls the LLM unless the user explicitly confirmed a preview.
- Clamp request sizes: topic length, note count, sentence count, max known words injected into prompts.

### C2. AI Services

```
src/lib/server/services/ai/
  course-generator.ts    — topic → course + vocabulary lessons + notes
  word-qa.ts             — question + context → answer
  sentence-generator.ts  — known words → practice sentences/conversations
```

Each service builds a system prompt + user prompt, calls `callLlmWithRetry()`, parses the JSON response with Zod, and returns typed data.

Recommended service contract:

```typescript
type AiPreview<T> = {
  data: T
  model: string
  attempts: number
  warnings: string[]
}
```

Warnings are for non-fatal cleanup, e.g. duplicate terms removed, missing furigana fallback, or generated sentences that use words outside the allowed vocabulary.

#### Course Generator Prompt Design

**System prompt:**

```
You are a language curriculum designer. Given a topic, level, and target language,
generate a structured vocabulary course.

Return a JSON object with this exact shape:
{
  "title": "string",
  "description": "string",
  "lessons": [
    {
      "title": "string",
      "type": "vocabulary",
      "notes": [
        {
          "term": "target language word",
          "reading": "pronunciation if applicable",
          "furigana": "furigana reading if applicable (for Japanese)",
          "definition": "English meaning",
          "example": "example sentence in target language",
          "example_translation": "English translation of example",
          "part_of_speech": "noun/verb/adjective/etc",
          "tags": ["tag1", "tag2"]
        }
      ]
    }
  ]
}

Rules:
- For Japanese, term = kanji form, reading = hiragana, furigana = reading annotation for kanji
- For Japanese, return kana-only readings in `reading`; never put romaji there
- For Japanese, return `furigana` using bracket notation like `食[た]べる`; do not return HTML
- Every note MUST have a definition and term at minimum
- Examples should use vocabulary the learner would know at this level
- Tags should include the JLPT level (e.g., "N5") and thematic tag (e.g., "food")
- Deduplicate terms inside a lesson
- Do NOT include explanations or commentary — only the JSON
```

#### Word Q&A Prompt Design

**System prompt:**

```
You are a knowledgeable language tutor. Answer the user's question about vocabulary
words, their usage, nuances, grammar points, or cultural context.

Below are the words the user is currently studying:
[injected list of terms, readings, definitions]

When answering:
- Reference the user's known words when helpful
- Provide example sentences using the words in question
- Explain nuances between similar words if relevant
- Be concise but thorough
- Use the target language script (kanji/hiragana for Japanese) with readings
- Do not create new notes or cards from Q&A answers; Q&A is read-only until the user explicitly chooses "Add to course"
```

#### Sentence Generator Prompt Design

**System prompt (sentences mode):**

```
Generate practice sentences using ONLY the following vocabulary words:
[injected vocabulary list]

Return a JSON object with this shape:
{
  "sentences": [
    {
      "sentence": "target language sentence",
      "translation": "English translation",
      "word_blocks": ["word1", "word2", "word3", ...],
      "used_words": ["term1", "term2"],
      "grammar_point": "optional grammar note"
    }
  ]
}

Rules:
- Use ONLY words from the provided list (or very obvious conjugations)
- Word blocks should be individual words/phrases that can be reordered for sentence_order practice
- Sentences should be level-appropriate and natural
- Aim for 5-10 sentences
- For Japanese, keep particles as separate blocks when that makes ordering practice clearer
```

**System prompt (conversation mode):**

```
Generate a short conversation using ONLY the following vocabulary words:
[injected vocabulary list]

Return a JSON object:
{
  "title": "descriptive title",
  "setting": "where this takes place",
  "dialogue": [
    { "speaker": "A", "sentence": "...", "translation": "..." },
    { "speaker": "B", "sentence": "...", "translation": "..." }
  ],
  "used_words": ["term1", "term2"]
}

Rules:
- 4-8 turns of dialogue
- Natural, everyday conversation
- Use ONLY words from the provided list
- Each sentence should be short enough to practice as a sentence_order card
```

---

## Part D: Card Generation Strategy

When notes are created (manually or via AI), cards are generated based on **course display options**.

### For `vocabulary` lessons:

```
note → cards:
  recognition          (term → definition)       [always]
  recall               (definition → term)        [always]
  reading_recognition  (kanji → hiragana)         [if showHiragana && showKanji]
```

Multiple-choice and typing cards are post-v1. Multiple-choice distractors should eventually be generated deterministically from other notes in the same course where possible. Only ask the LLM for distractors when the course has too few notes.

### For `sentence_practice` lessons:

```
note (with example sentence) → cards:
  sentence_order       (translation → reorder words)  [always]
  sentence_cloze       (sentence with blank → word)   [always]
```

Sentence practice notes can use:

- `term`: target sentence
- `reading`: sentence reading, if generated
- `furigana`: sentence furigana markup
- `definition`: English translation
- `example`: optional grammar note or alternate sentence
- `tags`: `["sentence-practice", ...usedWords]`

### For `conversation` lessons:

```
Each dialogue line → cards:
  sentence_order       (translate → reorder)
  sentence_cloze       (line with blank → missing word)
```

---

## Part E: API Routes & UI Pages

### API routes (server endpoints):

| Route                           | Method | Purpose                                                     |
| ------------------------------- | ------ | ----------------------------------------------------------- |
| `/api/ai/status`                | GET    | Check if LLM is configured + return model info              |
| `/api/ai/generate-course`       | POST   | Generate course + lessons + notes from a topic              |
| `/api/ai/generate-sentences`    | POST   | Generate sentences/conversations using known words          |
| `/api/ai/ask-about-words`       | POST   | Answer questions about vocabulary in context                |

All AI routes require authentication and are rate-limited (5 req/min per user, in-memory sliding window).

Route behavior:

- `generate-course`, `generate-sentences`, and `ask-about-words` return previews only.
- Confirm/create actions should reuse existing SvelteKit form actions on app pages so persistence stays protected by page-level course ownership checks.
- API routes should never trust `courseId` alone; always load the course by `(courseId, ownerId)`.
- `/api/ai/status` returns `{ configured, model, baseUrlHost }` and never returns an API key.

### New UI pages:

| Route                               | Purpose                                                                  |
| ----------------------------------- | ------------------------------------------------------------------------ |
| `/app/courses/ai-generate` | AI course generator — topic input, level, card count, preview before creating |
| `/app/courses/[id]/ai`             | In-course AI panel — generate sentences, ask questions, add more vocab   |
| `/app/courses/[id]/settings`       | Course settings — display options, card types, language profile          |

### Updated existing pages:

| Route                                 | Changes                                                                         |
| ------------------------------------- | ------------------------------------------------------------------------------- |
| `/app/courses/[id]`                   | Add furigana column to notes list, "AI Tools" button, group notes by lesson     |
| `/app/courses/[id]/notes/new`         | Add furigana field, show/hide based on `language_profile`                       |
| `/app/courses/[id]/review`            | Render all card types (not just flip): multiple choice grid, typing input, sentence ordering |
| `/app/courses/[id]/review`            | Respect display options — hide/show kanji/hiragana/furigana/english per card    |

---

## Part F: Implementation Order (Milestones)

### M1: Foundation — LLM Client + Env — Done

- Done: created `src/lib/server/services/llm.ts` from the lightweight utilities client.
- Done: `loadLlmConfig()` lives beside the LLM client; rate-limit env parsing lives in `src/lib/server/services/ai/rate-limit.ts`.
- Done: created `src/lib/server/services/ai/json.ts` for JSON extraction and Zod error formatting.
- Done: added env vars to `.env.example` and `docker-compose.yml`.
- Done: added `/api/ai/status`.
- Done: verified llama.cpp connectivity from the workspace and from a one-off `srs` Docker Compose container.
- Done: added focused unit tests for base URL normalization, JSON extraction, validation failures, route behavior, and rate limiting.
- Remaining: timeout behavior is covered by the LLM implementation but does not yet have a dedicated timeout test.

### M2: Schema Migrations — Done

- Done: added `furigana` to `note`.
- Done: added `type` and `metadata` to `lesson`.
- Done: added `display_options` and `language_profile` to `course`.
- Done: updated Zod validators for notes and courses.
- Done: updated `src/lib/server/db/schema.ts` for JSONB fields.
- Done: migration added as `drizzle/0002_ai_language_profiles.sql`.
- Remaining: there is no separate `lesson.ts` validator because v1 lesson persistence is driven through validated AI preview schemas and page actions.
- Remaining: run the migration against the target deployed database before release.

### M3: Japanese Adapter + Display Options — Done

- Done: created `src/lib/language/japanese.ts`.
- Done: created `src/lib/language/profiles.ts` for language profiles and display-option normalization.
- Done: created `src/lib/cards/kinds.ts` for valid card kinds, labels, and card builders.
- Done: updated note form labels and fields for Japanese courses.
- Done: updated note list rendering with safe furigana/ruby display.
- Done: added course detail settings controls for display toggles and active card kinds.
- Done: added repository support for updating course display options.

### M4: AI Course Generator — Done

- Done: course generation is implemented in `src/lib/server/services/ai/generate.ts`.
- Done: added `/api/ai/generate-course`.
- Done: added AI course generation UI on `/app/courses/new` with topic input, preview, and confirmed create.
- Done: added transactional generated-course persistence in `src/lib/server/services/ai/persist.ts`.
- Done: handles JSON parsing and Zod validation before route responses.
- Done: confirmed action persists the preview and creates initial `review_state` rows for every card.
- Remaining: preview warnings and duplicate-term cleanup are not implemented yet.

### M5: Scoped Discussion — Done

- Done: discussion is implemented in `src/lib/server/services/ai/generate.ts`.
- Done: added `/api/ai/discuss-content`.
- Done: added a read-only discussion panel on the course detail page.
- Done: discussion scopes include course, note, sentence, and conversation context.
- Done: discussion history is not saved in v1.

### M6: Sentence & Conversation Generator — Done

- Done: dynamic practice generation is implemented in `src/lib/server/services/ai/generate.ts`.
- Done: added `/api/ai/generate-practice` with `sentences` and `conversation` modes.
- Done: generation uses learned words by default via `getLearnedNotesForCourse`; selected notes override that; all course notes are the fallback.
- Done: confirmed persistence creates `sentence_practice` or `conversation` lessons with notes.
- Done: sentence and conversation notes generate `sentence_order` and `sentence_cloze` cards.
- Done: preview rows that are not confirmed are discarded.
- Remaining: no separate `/app/courses/[id]/ai` tools page exists; the v1 UI lives on the course detail page.

### M7: Multi-Card-Type Review UI — Done

Interactive study behavior implemented for all v1 card kinds:

- Done: `recognition` and `recall` — existing reveal/rating UI preserved.
- Done: `reading_recognition` — existing reveal/rating UI preserved.
- Done: `sentence_order` — clickable word blocks to build/reorder, submit to grade locally, correct/incorrect feedback, auto-rate (good/again) through existing `rate` action. Word blocks shuffled client-side via Fisher-Yates.
- Done: `sentence_cloze` — sentence with highlighted blank, text input, grade with `normalizeJapaneseAnswer()` for Japanese courses, feedback, auto-rate through existing `rate` action.
- Later: `multiple_choice` remains post-v1.
- Later: `typing` remains post-v1.
- Later: `listening` remains post-v1.

Keyboard shortcuts:
- Reveal cards: `Space`/`Enter` to flip, `1`-`4` to rate.
- Sentence order: `Enter` to submit when all blocks placed.
- Sentence cloze: `Enter` to submit from input.
- After auto-grade feedback: `Space`/`Enter` to skip the 1.2s auto-advance delay.

**Files delivered:** `src/lib/cards/grading.ts`, `src/lib/cards/grading.test.ts`, `src/routes/app/courses/[id]/review/+page.svelte` (rewrite).

### M8: UI Tests, Polish & Deploy — Next

- Done: graceful inline fallback when AI generation/discussion fails.
- Done: rate-limit AI endpoints with a simple in-memory user/scope sliding window.
- Done: Docker connectivity check from a one-off `srs` container to llama.cpp.
- Next: add Playwright config and e2e tests for Japanese course creation, display toggles, AI preview flows, and multi-card review behavior.
  - Package has `@playwright/test` installed and a `test:e2e` script, but no `playwright.config.*` or e2e test directory exists.
  - Setup requires: Playwright config with dev server URL, auth helper (seed user or login flow), and deterministic test data.
- Next: add loading skeletons or stronger pending states for AI requests.
- Next: add route-level error boundaries where AI panels live.
- Next: add basic observability for model, latency, attempts, validation failures, and rate-limit denials without logging full user prompts by default.
- Later: streaming AI responses.
- Later: premium/local LLM fallback.
- **Estimated files:** 2-4 new test/config files, 3 edited UI/service files

---

## File Tree — Implemented V1 Files

```
src/lib/
  cards/
    grading.ts                                        ← DONE Answer-grading helpers + shuffle utilities
    grading.test.ts                                   ← DONE 17 grading tests
    kinds.ts                                           ← DONE Valid card kinds, labels, card builders
    kinds.test.ts                                      ← DONE Card builder tests
  language/
    japanese.ts                                       ← DONE Japanese utilities
    japanese.test.ts                                  ← DONE Japanese utility tests
    profiles.ts                                       ← DONE Language profile + display option registry

src/lib/server/services/
  llm.ts                                              ← DONE LLM client (ported from utilities)
  llm.test.ts                                         ← DONE LLM helper tests
  ai/
    generate.ts                                       ← DONE Course/practice/discussion generation
    json.ts                                           ← DONE JSON extraction + validation helpers
    json.test.ts                                      ← DONE JSON/validation tests
    persist.ts                                        ← DONE Confirmed persistence helpers
    rate-limit.ts                                     ← DONE User/scope rate limiting
    rate-limit.test.ts                                ← DONE Rate-limit tests
    schemas.ts                                        ← DONE AI preview schemas

src/lib/server/db/
  schema.ts                                           ← DONE furigana, lesson.type, lesson.metadata, course.display_options, course.language_profile

src/lib/validation/
  note.ts                                             ← DONE add furigana
  course.ts                                           ← DONE add displayOptions, languageProfile

src/routes/api/ai/
  status/+server.ts                                   ← DONE
  generate-course/+server.ts                          ← DONE preview-only course generation
  generate-course/server.test.ts                      ← DONE route tests
  generate-practice/+server.ts                        ← DONE preview-only sentence/conversation generation
  generate-practice/server.test.ts                    ← DONE route tests
  discuss-content/+server.ts                          ← DONE scoped read-only discussion
  discuss-content/server.test.ts                      ← DONE route tests

src/lib/server/repositories/
  courses.ts                                          ← DONE display options and language profile
  notes.ts                                            ← DONE furigana, card builders, learned-word lookup

src/routes/app/courses/
  new/+page.svelte                                    ← DONE manual + AI course creation UI
  new/+page.server.ts                                 ← DONE manual create + confirmed AI create actions
  [id]/+page.svelte                                   ← DONE furigana display, settings, AI practice/discussion panels
  [id]/+page.server.ts                                ← DONE settings + confirmed practice persistence actions
  [id]/notes/new/+page.svelte                         ← DONE furigana/Japanese fields
  [id]/notes/new/+page.server.ts                      ← DONE furigana persistence + profile-aware card creation
  [id]/review/+page.svelte                            ← PARTIAL labels/display for v1 kinds; interactive sentence cards remain
  [id]/review/+page.server.ts                         ← DONE card ownership check and display options

drizzle/
  0002_ai_language_profiles.sql                       ← DONE migration

.env.example                                          ← DONE add LLM vars
docker-compose.yml                                    ← DONE add LLM vars to srs service
vitest.config.ts                                      ← DONE $lib alias for tests
```

---

## Reference: Existing LLM Setup (Utilities)

The utilities app at `~/Code/dokploy-apps/utilities/` has a proven LLM integration pattern. The user referred to `dockploy-apps`; the local checked path is `dokploy-apps`.

| File | Purpose |
| ---- | ------- |
| `server/llm.ts` | `callLlm()` + `callLlmWithRetry()` — OpenAI-compatible API client with timeout, error handling, reasoning model support |
| `server/ai-agents.ts` | Multi-provider agent system with DB persistence, rate limiting, API key encryption. Overkill for SRS — we use the simpler `llm.ts` directly. |
| `server/premium-llm.ts` | Premium → local LLM fallback chain. `callAi()` tries premium first, falls back to local. Useful pattern we can adapt. |
| `server/prompting.ts` | System/user prompt builder with few-shot examples. Good reference for prompt structure. |

**Current config (working with llamacpp):**

```
LLM_BASE_URL=http://10.88.111.16:8080
LLM_MODEL=qwen3.5-4b
LLM_TIMEOUT_MS=60000
```

For Docker deployment, the SRS container will need to reach the host's llamacpp server. Options:
- `LLM_BASE_URL=http://host.docker.internal:8080` (Docker Desktop / rootless)
- `LLM_BASE_URL=http://10.88.111.16:8080` (direct IP, works if network allows)
- Use `network_mode: host` on the SRS service

## Implementation Notes From Current SRS App

The current SRS app is now a SvelteKit/Drizzle app with AI/Japanese v1 support layered onto the original repositories:

- `src/lib/server/repositories/courses.ts` supports `languageProfile` and `displayOptions`.
- `src/lib/server/repositories/notes.ts` supports `furigana`, profile-aware vocabulary card creation, and learned-word lookup for AI practice generation.
- `lesson.type` and `lesson.metadata` are used for generated vocabulary, sentence-practice, and conversation lessons.
- AI endpoints return previews only. Confirmed persistence runs through page actions after auth and ownership checks.
- Review loading joins `cards`, `notes`, and `review_state`, and now includes course display options for card rendering.
- The main remaining implementation gap is interactive review behavior for `sentence_order` and `sentence_cloze`.

Because of that, the safest next implementation sequence is review helper logic -> review UI interactions -> focused tests -> optional Playwright smoke coverage.

## Continuation Prompt

Use this prompt for the next implementation session:

```text
Continue work in `/home/cory/Code/temp-repo-cleanup/demos/apps/spaced-repitition`.

The SRS Japanese + AI v1 foundation is implemented. The interactive multi-card-type review UI (M7) is complete. Read `docs/ai-features-plan.md` for current status.

Goal for the next phase (M8):
Add Playwright config and e2e tests for the main Japanese and AI preview flows, plus UI polish.

Requirements:
- Create `playwright.config.ts` with the dev server URL and webServer config.
- Add an auth helper (seed a test user or use register/login flows).
- Write e2e specs for Japanese course creation, display toggles, AI preview flows, and multi-card review behavior (all v1 card kinds).
- Add loading skeletons or stronger pending states for AI requests.
- Add route-level error boundaries where AI panels live.
- Add basic observability (model latency, retry attempts, validation failures, rate-limit denials) without logging full user prompts.

Verification:
- Run `pnpm test`.
- Run `pnpm run check`.
- Run `pnpm run build`.
- Run `pnpm run test:e2e`.
- Report anything that could not be run.

Important constraints:
- Do not revert unrelated user changes in this repo.
- Use existing SvelteKit, Drizzle, and repository patterns.
- Keep changes scoped and testable.
```
