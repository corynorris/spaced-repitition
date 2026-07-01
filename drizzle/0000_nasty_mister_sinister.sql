CREATE TYPE "public"."card_state" AS ENUM('new', 'learning', 'review', 'relearning', 'suspended');--> statement-breakpoint
CREATE TYPE "public"."review_rating" AS ENUM('again', 'hard', 'good', 'easy');--> statement-breakpoint
CREATE TABLE "card" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"note_id" uuid NOT NULL,
	"kind" text NOT NULL,
	"prompt" text NOT NULL,
	"answer" text NOT NULL,
	"extra" jsonb DEFAULT '{}'::jsonb NOT NULL,
	"is_suspended" boolean DEFAULT false NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL,
	"updated_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "course" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"owner_id" text NOT NULL,
	"title" text NOT NULL,
	"description" text,
	"source_language" text,
	"target_language" text,
	"is_archived" boolean DEFAULT false NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL,
	"updated_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "lesson" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"course_id" uuid NOT NULL,
	"title" text NOT NULL,
	"order_index" integer DEFAULT 0 NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL,
	"updated_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "note_tag" (
	"note_id" uuid NOT NULL,
	"tag" text NOT NULL,
	CONSTRAINT "note_tag_note_id_tag_pk" PRIMARY KEY("note_id","tag")
);
--> statement-breakpoint
CREATE TABLE "note" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"course_id" uuid NOT NULL,
	"lesson_id" uuid,
	"term" text NOT NULL,
	"reading" text,
	"definition" text NOT NULL,
	"example" text,
	"example_translation" text,
	"part_of_speech" text,
	"notes" text,
	"tags" text[] DEFAULT '{}' NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL,
	"updated_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "review_log" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"card_id" uuid NOT NULL,
	"rating" "review_rating" NOT NULL,
	"reviewed_at" timestamp with time zone DEFAULT now() NOT NULL,
	"previous_state" jsonb NOT NULL,
	"next_state" jsonb NOT NULL
);
--> statement-breakpoint
CREATE TABLE "review_state" (
	"card_id" uuid PRIMARY KEY NOT NULL,
	"state" "card_state" DEFAULT 'new' NOT NULL,
	"due_at" timestamp with time zone DEFAULT now() NOT NULL,
	"stability" integer,
	"difficulty" integer,
	"elapsed_days" integer DEFAULT 0 NOT NULL,
	"scheduled_days" integer DEFAULT 0 NOT NULL,
	"reps" integer DEFAULT 0 NOT NULL,
	"lapses" integer DEFAULT 0 NOT NULL,
	"updated_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
ALTER TABLE "card" ADD CONSTRAINT "card_note_id_note_id_fk" FOREIGN KEY ("note_id") REFERENCES "public"."note"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "lesson" ADD CONSTRAINT "lesson_course_id_course_id_fk" FOREIGN KEY ("course_id") REFERENCES "public"."course"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "note_tag" ADD CONSTRAINT "note_tag_note_id_note_id_fk" FOREIGN KEY ("note_id") REFERENCES "public"."note"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "note" ADD CONSTRAINT "note_course_id_course_id_fk" FOREIGN KEY ("course_id") REFERENCES "public"."course"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "note" ADD CONSTRAINT "note_lesson_id_lesson_id_fk" FOREIGN KEY ("lesson_id") REFERENCES "public"."lesson"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "review_log" ADD CONSTRAINT "review_log_card_id_card_id_fk" FOREIGN KEY ("card_id") REFERENCES "public"."card"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "review_state" ADD CONSTRAINT "review_state_card_id_card_id_fk" FOREIGN KEY ("card_id") REFERENCES "public"."card"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
CREATE UNIQUE INDEX "card_note_kind_idx" ON "card" USING btree ("note_id","kind");--> statement-breakpoint
CREATE INDEX "card_note_idx" ON "card" USING btree ("note_id");--> statement-breakpoint
CREATE UNIQUE INDEX "course_owner_title_idx" ON "course" USING btree ("owner_id","title");--> statement-breakpoint
CREATE INDEX "course_owner_idx" ON "course" USING btree ("owner_id");--> statement-breakpoint
CREATE UNIQUE INDEX "lesson_course_order_idx" ON "lesson" USING btree ("course_id","order_index");--> statement-breakpoint
CREATE INDEX "lesson_course_idx" ON "lesson" USING btree ("course_id");--> statement-breakpoint
CREATE INDEX "note_tag_tag_idx" ON "note_tag" USING btree ("tag");--> statement-breakpoint
CREATE INDEX "note_course_term_idx" ON "note" USING btree ("course_id","term");--> statement-breakpoint
CREATE INDEX "note_lesson_idx" ON "note" USING btree ("lesson_id");--> statement-breakpoint
CREATE INDEX "review_log_card_reviewed_at_idx" ON "review_log" USING btree ("card_id","reviewed_at");