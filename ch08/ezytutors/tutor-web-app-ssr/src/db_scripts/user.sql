/* Create the table for users */
CREATE TABLE ezyweb_user (
  username varchar(20) PRIMARY KEY,
  tutor_id INT,
  user_password CHAR(100) NOT NULL
);