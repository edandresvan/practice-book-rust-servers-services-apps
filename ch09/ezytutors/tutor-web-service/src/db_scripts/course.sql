/* Create the table for courses */
CREATE TABLE course_ch06 (
  course_id serial PRIMARY KEY,
  tutor_id integer NOT NULL,
  course_name varchar(140) NOT NULL,
  posted_time timestamp default now(),
  course_description varchar(2000),
  course_format varchar(30),
  course_structure varchar(200),
  course_duration varchar(30),
  course_price integer,
  course_language varchar(30),
  course_level varchar(30)
);

/* Add constraints */

ALTER TABLE course_ch06 
  ADD CONSTRAINT course_fk_tutor_id 
  FOREIGN KEY (tutor_id) 
  REFERENCES tutor_ch06 (tutor_id) 
  ON DELETE RESTRICT;


