## safe-index-rs

### Brief description

Small rust project using `celery` to provide tasks consulting events from an RPC node for later processing and storage in some form of database.

### Usage

Make sure to have lunched your `broker`. For the sake of testing you can use the script in the parent folder like so:

```bash
$ sh ../scripts/broker.sh
```

Also, provide the environment variables as shown in the `.env.example` file. 

To run the `producer` use:

```bash
$ cargo run --bin consumer
# alternatively, if you are using cargo-watch
$ cargo watch -x "run --bin consumer"
```

To run the `producer` use:

```bash
$ cargo run --bin producer
# alternatively, if you are using cargo-watch
$ cargo watch -x "run --bin producer"
```