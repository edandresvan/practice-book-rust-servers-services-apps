 Practice Book: Rust Web Development


This repository contains my practical exercises from the book ["Rust Servers, Services, and Apps" by Prabhu Eshwarla (Manning)](https://www.manning.com/books/rust-servers-services-and-apps).


![Book Cover](https://images.manning.com/360/480/resize/book/9/03ac487-c409-4b45-ac49-8affc8b524fe/Eshwarla-RSSA-MEAPHI.png)

The original code repository is also located [here in GitHub](https://github.com/peshwar9/rust-servers-services-apps).

## License

[MIT](https://choosealicense.com/licenses/mit/)


The files in this repository are my own practice following the book lessons.|

However, the original copyright belongs to Manning Books.

Eshwarla, Prabhu. 2023. Rust Servers, Services, and Apps. Manning Publications. ISBN: 9781617298608

## PostgreSQL Installation

From chapter 4, a PostgreSQL database is used as the datastore for the web service. To install PostgreSQL using a [Podman](https://podman.io/) container and Ubuntu 22.04 follow these steps:

### Install Podman

```bash
$ sudo apt-get install aptitude;
$ sudo aptitude update && sudo aptitude upgrade -y;
$ sudo aptitude install podman;
```

### Install PostgreSQL Clients

```bash

$ wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo gpg --dearmor -o /usr/share/keyrings/postgresql.gpg;
$ sudo sh -c 'echo "deb [signed-by=/usr/share/keyrings/postgresql.gpg] http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/postgresql.list';

$ wget --quiet -O - https://www.pgadmin.org/static/packages_pgadmin_org.pub | sudo gpg --dearmor -o /usr/share/keyrings/packages-pgadmin-org.gpg;

$ sudo sh -c 'echo "deb [signed-by=/usr/share/keyrings/packages-pgadmin-org.gpg] https://ftp.postgresql.org/pub/pgadmin/pgadmin4/apt/$(lsb_release -cs) pgadmin4 main" > /etc/apt/sources.list.d/pgadmin4.list';
 
$ sudo aptitude update && sudo aptitude upgrade -y;

$ sudo aptitude install libpq-dev postgresql-client pgadmin4-desktop -y;
```

### Install a PostgreSQL Container

We will use the PostgreSQL container based on Debian Linux from the Docker Registry.

Create a disk volume (i.e. a directory) to persist the database we will maintain in the container.

```bash
$ podman volume create ezytutors_volume;
```

Create the PostgreSQL container with the previously created disk volume and the customizable parameters. The first time, we specify the `root` password.

```bash
$ podman run --interactive --publish 5432:5432 --volume ezytutors_volume:/var/lib/postgresql/data --memory 500m --env POSTGRES_PASSWORD=myP4ssw0rd --name ezytutors docker.io/library/postgres:15-bullseye;
```

### Create the PostgreSQL Database for the EzyTutors Service (Back-end)

```bash
$ psql --host=localhost --port=5432 --username=postgres --password;


postgres=# create database ezytutors_db;
postgres=# create user truuser with password 'mypassword';
postgres=# grant all privileges on database ezytutors_db to truuser;
postgres=# \q

```

```bash
$ psql --host=localhost --port=5432 --dbname=ezytutors_db --username=truuser --password;
```

### Create the PostgreSQL Database for the SSR Front-end

```bash
$ psql --host=localhost --port=5432 --username=postgres --password;


postgres=# create database ezytutors_web_ssr_db;
postgres=# create user ssruser with password 'mypassword';
postgres=# grant all privileges on database ezytutors_web_ssr_db to ssruser;
postgres=# \q

```

```bash
$ psql --host=localhost --port=5432 --ezytutors_web_ssr_db --username=ssruser --password;
```

### Execute PostgreSQL Scripts

```bash
$ cd src/db-scripts;
$ psql --host=localhost --port=5432 --dbname=ezytutors_db --username=truuser --password --file=src/database.sql;
``` 

```bash
$ cd src/db-scripts;
$ psql --host=localhost --port=5432 --user=truuser --dbname=ezytutors_db --file=db-create.sql
```







