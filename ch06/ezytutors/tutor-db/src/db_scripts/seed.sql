/* Seed data */

insert into tutor_ch06(
    tutor_id,
    tutor_name,
    tutor_pic_url,
    tutor_profile
  )
values(
    1,
    'Merlene',
    'http://s3.amazon.aws.com/pic1',
    'Merlene is an experienced finance professional'
  );
insert into tutor_ch06(
    tutor_id,
    tutor_name,
    tutor_pic_url,
    tutor_profile
  )
values(
    2,
    'Frank',
    'http://s3.amazon.aws.com/pic2',
    'Frank is an expert nuclear engineer'
  );
insert into tutor_ch06(
    tutor_id,
    tutor_name,
    tutor_pic_url,
    tutor_profile
  )
values(
    3,
    'Bob',
    'http://s3.amazon.aws.com/pic3',
    'Bob has spent many years teaching ML to students and professionals alike'
  );
insert into course_ch06 (
    course_id,
    tutor_id,
    course_name,
    course_level,
    posted_time
  )
values(
    1,
    1,
    'First course',
    'Beginner',
    '2021-04-12 05:40:00'
  );
insert into course_ch06 (
    course_id,
    tutor_id,
    course_name,
    course_format,
    posted_time
  )
values(
    2,
    2,
    'Second course',
    'ebook',
    '2021-04-12 05:45:00'
  );
insert into course_ch06 (
    course_id,
    tutor_id,
    course_name,
    course_format,
    posted_time
  )
values(
    3,
    1,
    'Second course from author 1',
    'ebook',
    '2021-04-12 05:45:00'
  );
insert into course_ch06 (
    course_id,
    tutor_id,
    course_name,
    course_format,
    posted_time
  )
values(
    4,
    1,
    'Third course from author 1',
    'ebook',
    '2021-04-12 05:45:00'
  );
insert into course_ch06 (
    course_id,
    tutor_id,
    course_name,
    course_format,
    posted_time
  )
values(
    5,
    3,
    'First course from author 3',
    'ebook',
    '2021-04-12 05:45:00'
  )