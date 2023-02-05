-- Create the table for tutors
CREATE TABLE tutor_ch06 (
  tutor_id serial PRIMARY KEY,
  tutor_name varchar(200) NOT NULL,
  tutor_pic_url varchar(200) NOT NULL,
  tutor_profile varchar(2000) NOT NULL
);
