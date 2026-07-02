ALTER TABLE "course" ADD COLUMN "language_profile" text DEFAULT 'generic' NOT NULL;--> statement-breakpoint
ALTER TABLE "course" ADD COLUMN "display_options" jsonb DEFAULT '{}'::jsonb NOT NULL;--> statement-breakpoint
ALTER TABLE "lesson" ADD COLUMN "type" text DEFAULT 'vocabulary' NOT NULL;--> statement-breakpoint
ALTER TABLE "lesson" ADD COLUMN "metadata" jsonb DEFAULT '{}'::jsonb NOT NULL;--> statement-breakpoint
ALTER TABLE "note" ADD COLUMN "furigana" text;
