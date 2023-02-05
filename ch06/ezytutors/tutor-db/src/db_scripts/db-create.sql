BEGIN;

-- Drop tables if they already exist
DROP TABLE IF EXISTS course_ch06;
DROP TABLE IF EXISTS tutor_ch06;

/* Create tables */
\i tutor.sql;
\i course.sql;

COMMIT;