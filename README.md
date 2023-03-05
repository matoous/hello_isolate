# Hello Isolate!

Application that prints `Hello World!` from a JavaScript script
running in [V8 Isolate](https://v8docs.nodesource.com/node-0.8/d5/dda/classv8_1_1_isolate.html)
in [Rust](https://www.rust-lang.org/).

This repository contains the code for the [How do CloudFlare Workers work?](https://dzx.cz/2023/03/08/how_do_cloudflare_workers_work/)
blog post.

The `main` branch has simple example for _Hello World!_.
You can find slightly more complex example that compiles the script as a worker
and provides basic runtime in the [hello-runtime](https://github.com/matoous/hello_isolate/tree/hello-runtime) branch.

## Run

Run using

```sh
cargo run
```
