import {
	boolean,
	index,
	integer,
	jsonb,
	pgEnum,
	pgTable,
	primaryKey,
	text,
	timestamp,
	uniqueIndex,
	uuid,
} from "drizzle-orm/pg-core";

export const cardState = pgEnum("card_state", [
	"new",
	"learning",
	"review",
	"relearning",
	"suspended",
]);

export const reviewRating = pgEnum("review_rating", [
	"again",
	"hard",
	"good",
	"easy",
]);

// ═══════════════════════════════════════════════
// Better Auth tables (managed by Drizzle adapter)
// ═══════════════════════════════════════════════

export const user = pgTable("user", {
	id: text("id").primaryKey(),
	name: text("name").notNull(),
	email: text("email").notNull().unique(),
	emailVerified: boolean("email_verified").notNull(),
	image: text("image"),
	createdAt: timestamp("created_at", { withTimezone: true }).notNull(),
	updatedAt: timestamp("updated_at", { withTimezone: true }).notNull(),
});

export const session = pgTable("session", {
	id: text("id").primaryKey(),
	expiresAt: timestamp("expires_at", { withTimezone: true }).notNull(),
	token: text("token").notNull().unique(),
	createdAt: timestamp("created_at", { withTimezone: true }).notNull(),
	updatedAt: timestamp("updated_at", { withTimezone: true }).notNull(),
	ipAddress: text("ip_address"),
	userAgent: text("user_agent"),
	userId: text("user_id")
		.notNull()
		.references(() => user.id, { onDelete: "cascade" }),
});

export const account = pgTable("account", {
	id: text("id").primaryKey(),
	accountId: text("account_id").notNull(),
	providerId: text("provider_id").notNull(),
	userId: text("user_id")
		.notNull()
		.references(() => user.id, { onDelete: "cascade" }),
	accessToken: text("access_token"),
	refreshToken: text("refresh_token"),
	idToken: text("id_token"),
	accessTokenExpiresAt: timestamp("access_token_expires_at", {
		withTimezone: true,
	}),
	refreshTokenExpiresAt: timestamp("refresh_token_expires_at", {
		withTimezone: true,
	}),
	scope: text("scope"),
	password: text("password"),
	createdAt: timestamp("created_at", { withTimezone: true }).notNull(),
	updatedAt: timestamp("updated_at", { withTimezone: true }).notNull(),
});

export const verification = pgTable("verification", {
	id: text("id").primaryKey(),
	identifier: text("identifier").notNull(),
	value: text("value").notNull(),
	expiresAt: timestamp("expires_at", { withTimezone: true }).notNull(),
	createdAt: timestamp("created_at", { withTimezone: true }),
	updatedAt: timestamp("updated_at", { withTimezone: true }),
});

export const courses = pgTable(
	"course",
	{
		id: uuid("id").defaultRandom().primaryKey(),
		ownerId: text("owner_id").notNull(),
		title: text("title").notNull(),
		description: text("description"),
		sourceLanguage: text("source_language"),
		targetLanguage: text("target_language"),
		languageProfile: text("language_profile").notNull().default("generic"),
		displayOptions: jsonb("display_options")
			.$type<Record<string, unknown>>()
			.notNull()
			.default({}),
		isArchived: boolean("is_archived").notNull().default(false),
		createdAt: timestamp("created_at", { withTimezone: true })
			.notNull()
			.defaultNow(),
		updatedAt: timestamp("updated_at", { withTimezone: true })
			.notNull()
			.defaultNow(),
	},
	(table) => ({
		ownerTitle: uniqueIndex("course_owner_title_idx").on(
			table.ownerId,
			table.title,
		),
		owner: index("course_owner_idx").on(table.ownerId),
	}),
);

export const lessons = pgTable(
	"lesson",
	{
		id: uuid("id").defaultRandom().primaryKey(),
		courseId: uuid("course_id")
			.notNull()
			.references(() => courses.id, { onDelete: "cascade" }),
		title: text("title").notNull(),
		type: text("type").notNull().default("vocabulary"),
		metadata: jsonb("metadata")
			.$type<Record<string, unknown>>()
			.notNull()
			.default({}),
		orderIndex: integer("order_index").notNull().default(0),
		createdAt: timestamp("created_at", { withTimezone: true })
			.notNull()
			.defaultNow(),
		updatedAt: timestamp("updated_at", { withTimezone: true })
			.notNull()
			.defaultNow(),
	},
	(table) => ({
		courseOrder: uniqueIndex("lesson_course_order_idx").on(
			table.courseId,
			table.orderIndex,
		),
		course: index("lesson_course_idx").on(table.courseId),
	}),
);

export const notes = pgTable(
	"note",
	{
		id: uuid("id").defaultRandom().primaryKey(),
		courseId: uuid("course_id")
			.notNull()
			.references(() => courses.id, { onDelete: "cascade" }),
		lessonId: uuid("lesson_id").references(() => lessons.id, {
			onDelete: "set null",
		}),
		term: text("term").notNull(),
		reading: text("reading"),
		furigana: text("furigana"),
		definition: text("definition").notNull(),
		example: text("example"),
		exampleTranslation: text("example_translation"),
		partOfSpeech: text("part_of_speech"),
		notes: text("notes"),
		tags: text("tags").array().notNull().default([]),
		createdAt: timestamp("created_at", { withTimezone: true })
			.notNull()
			.defaultNow(),
		updatedAt: timestamp("updated_at", { withTimezone: true })
			.notNull()
			.defaultNow(),
	},
	(table) => ({
		courseTerm: index("note_course_term_idx").on(table.courseId, table.term),
		lesson: index("note_lesson_idx").on(table.lessonId),
	}),
);

export const cards = pgTable(
	"card",
	{
		id: uuid("id").defaultRandom().primaryKey(),
		noteId: uuid("note_id")
			.notNull()
			.references(() => notes.id, { onDelete: "cascade" }),
		kind: text("kind").notNull(),
		prompt: text("prompt").notNull(),
		answer: text("answer").notNull(),
		extra: jsonb("extra")
			.$type<Record<string, unknown>>()
			.notNull()
			.default({}),
		isSuspended: boolean("is_suspended").notNull().default(false),
		createdAt: timestamp("created_at", { withTimezone: true })
			.notNull()
			.defaultNow(),
		updatedAt: timestamp("updated_at", { withTimezone: true })
			.notNull()
			.defaultNow(),
	},
	(table) => ({
		noteKind: uniqueIndex("card_note_kind_idx").on(table.noteId, table.kind),
		note: index("card_note_idx").on(table.noteId),
	}),
);

export const reviewStates = pgTable("review_state", {
	cardId: uuid("card_id")
		.primaryKey()
		.references(() => cards.id, { onDelete: "cascade" }),
	state: cardState("state").notNull().default("new"),
	dueAt: timestamp("due_at", { withTimezone: true }).notNull().defaultNow(),
	stability: integer("stability"),
	difficulty: integer("difficulty"),
	elapsedDays: integer("elapsed_days").notNull().default(0),
	scheduledDays: integer("scheduled_days").notNull().default(0),
	reps: integer("reps").notNull().default(0),
	lapses: integer("lapses").notNull().default(0),
	updatedAt: timestamp("updated_at", { withTimezone: true })
		.notNull()
		.defaultNow(),
});

export const reviewLogs = pgTable(
	"review_log",
	{
		id: uuid("id").defaultRandom().primaryKey(),
		cardId: uuid("card_id")
			.notNull()
			.references(() => cards.id, { onDelete: "cascade" }),
		rating: reviewRating("rating").notNull(),
		reviewedAt: timestamp("reviewed_at", { withTimezone: true })
			.notNull()
			.defaultNow(),
		previousState: jsonb("previous_state")
			.$type<Record<string, unknown>>()
			.notNull(),
		nextState: jsonb("next_state").$type<Record<string, unknown>>().notNull(),
	},
	(table) => ({
		cardReviewedAt: index("review_log_card_reviewed_at_idx").on(
			table.cardId,
			table.reviewedAt,
		),
	}),
);

export const noteTags = pgTable(
	"note_tag",
	{
		noteId: uuid("note_id")
			.notNull()
			.references(() => notes.id, { onDelete: "cascade" }),
		tag: text("tag").notNull(),
	},
	(table) => ({
		pk: primaryKey({ columns: [table.noteId, table.tag] }),
		tag: index("note_tag_tag_idx").on(table.tag),
	}),
);
