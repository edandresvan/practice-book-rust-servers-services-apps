BEGIN;

/* Drop tables if they already exist */
DROP TABLE IF EXISTS ezyweb_user;

/* Create tables */
\i user.sql;



COMMIT;