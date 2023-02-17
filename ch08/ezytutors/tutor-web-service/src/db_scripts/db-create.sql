BEGIN;

/* Drop tables if they already exist */
DROP TABLE IF EXISTS course_ch06;
DROP TABLE IF EXISTS tutor_ch06;

/* Create tables */
\i tutor.sql;
\i course.sql;

/* Grant privilegies */

GRANT ALL PRIVILEGES ON tutor_ch06 TO truuser;
GRANT ALL PRIVILEGES ON course_ch06 TO truuser;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO truuser;

/* Insert the seed data */
\i seed.sql;


COMMIT;
