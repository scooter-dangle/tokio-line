# Tokio Line Proto

This is a fork to experiment with a persistent echo server.

Build:

```sh
cargo build --release --example echo_server
cargo build --release --example echo_client
```

Run:

```sh
target/release/examples/echo_server
```

Observe the number of file handles opened by the echo server

```sh
watch --interval=0.1 "lsof -p $(pgrep echo_server) | wc --lines"
```

and the memory consumed by the process

```sh
top -pid $(pgrep echo_server)
# OR, depending on your version of `top`:
top -p$(pgrep echo_server)
```

Then, run the `echo_client` example:

```sh
for _ in $(seq 1000)
do
    target/release/examples/echo_client
done
```

On OS X, the echo server never releases the file handles. After enough runs of
the client, further attempts to connect to the server result in

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 49, message: "Can\'t assign requested address" } }', ../src/libcore/result.rs:788
```

It's possible that the [line](https://github.com/scooter-dangle/tokio-line/blob/master/examples/echo_server.rs#L25) used to cause the echo server to loop is at fault:

```rust
    core.run(futures::empty::<(), ()>()).unwrap();
```

If that's the issue, I'd still need to know the correct way to set the server up
to respond to more than only one request.


## License

Tokio is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
