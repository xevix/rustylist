Rustylist
=========

Todo list app to play around with Iron, Diesel etc.

**Requirements**

* Rust Nightly
* PostgreSQL

**Running**

The main Iron server app can be run as follows.

```
$ cargo run --bin server
```

It can be accessed at http://localhost:3000.

**Data Generation**

To add some todos from the CLI for now, run the following.

```
$ cargo run --bin write_todo
```
