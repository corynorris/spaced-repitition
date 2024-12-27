-- Create card types table for defining structured card formats
CREATE TABLE card_type (
    card_type_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT COLLATE case_insensitive NOT NULL UNIQUE,
    description TEXT,
    schema JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

SELECT trigger_updated_at('card_type');

-- Create courses table to organize learning material
CREATE TABLE course (
    course_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES "user"(user_id) ON DELETE CASCADE,
    title TEXT COLLATE case_insensitive NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, title)
);

SELECT trigger_updated_at('course');

-- Create lessons table for course structuring
CREATE TABLE lesson (
    lesson_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID NOT NULL REFERENCES course(course_id) ON DELETE CASCADE,
    title TEXT COLLATE case_insensitive NOT NULL,
    order_index INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (course_id, title),
    UNIQUE (course_id, order_index)
);

SELECT trigger_updated_at('lesson');

-- Create cards table for actual learning content
CREATE TABLE card (
    card_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lesson_id UUID NOT NULL REFERENCES lesson(lesson_id) ON DELETE CASCADE,
    card_type_id UUID NOT NULL REFERENCES card_type(card_type_id),
    content JSONB NOT NULL,
    order_index INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (lesson_id, order_index)
);

SELECT trigger_updated_at('card');

-- Create performance-oriented indexes
CREATE INDEX idx_course_user_id ON course(user_id);
CREATE INDEX idx_lesson_course_id ON lesson(course_id);
CREATE INDEX idx_card_lesson_id ON card(lesson_id);
CREATE INDEX idx_card_type_id ON card(card_type_id);

-- Create materialized view for course summaries
CREATE MATERIALIZED VIEW course_summary AS
SELECT 
    c.course_id,
    c.title,
    c.user_id,
    c.description,
    COUNT(DISTINCT l.lesson_id) as lesson_count,
    COUNT(card.card_id) as total_cards,
    c.created_at,
    MAX(c.updated_at) as last_updated
FROM course c
LEFT JOIN lesson l ON c.course_id = l.course_id
LEFT JOIN card ON l.lesson_id = card.lesson_id
GROUP BY c.course_id, c.title, c.user_id, c.description, c.created_at;

CREATE UNIQUE INDEX idx_course_summary_id ON course_summary(course_id);

-- Create refresh trigger function
CREATE OR REPLACE FUNCTION refresh_course_summary()
    RETURNS TRIGGER AS
$$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY course_summary;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Add refresh triggers
CREATE TRIGGER refresh_course_summary_on_course
    AFTER INSERT OR UPDATE OR DELETE ON course
    FOR EACH STATEMENT
    EXECUTE FUNCTION refresh_course_summary();

CREATE TRIGGER refresh_course_summary_on_lesson
    AFTER INSERT OR UPDATE OR DELETE ON lesson
    FOR EACH STATEMENT
    EXECUTE FUNCTION refresh_course_summary();

CREATE TRIGGER refresh_course_summary_on_card
    AFTER INSERT OR UPDATE OR DELETE ON card
    FOR EACH STATEMENT
    EXECUTE FUNCTION refresh_course_summary();