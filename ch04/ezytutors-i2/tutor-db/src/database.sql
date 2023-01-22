BEGIN;
-- Drop table if it already exists
DROP TABLE IF EXISTS course_ch04;
-- Create the table for courses
CREATE TABLE course_ch04 ( 
  course_id serial PRIMARY KEY,
  tutor_id int NOT NULL,
  course_name varchar(140) NOT NULL,
  posted_time timestamp default now() 
);

-- Seed data
INSERT INTO course_ch04 
  (course_id, tutor_id, course_name, posted_time)
  VALUES 
  (1, 1, 'First course', '2020-12-17 05:40:00'),
  (2, 1, 'Second course', '2020-12-18 05:45:00');

COMMIT;