Rustylist
=========

Todo list app to play around with Iron, Diesel etc.

**Requirements**

* Rust Nightly
* PostgreSQL

**Setup**

Add a `.env` file to the root directory with contents similar to the below pointing to your postgres instance and desired database name.

```
DATABASE_URL=postgres://localhost/rustylist
```

Then run migrations.

```bash
$ cargo install diesel_cli # just once
$ diesel setup # just once to create the db and run migrations
$ diesel migration run # to update thereafter
```

**Running**

The main Iron server app can be run as follows.

```bash
$ cargo run --bin server
```

It can be accessed at http://localhost:3000.

**Data Generation**

To add some todos from the CLI for now, run the following.

```bash
$ cargo run --bin write_todo
```
