# mal-rust

This is my own implementation of [mal](https://github.com/kanaka/mal) in Rust.

I wrote a little about my experience building this
[here](http://seven1m.sdf.org/experiments/make_a_lisp_in_rust.html).

The main mal repo already has a Rust implementation, so I'll keep this here.

## Build

This has been tested with Rust version 1.33.0.

```bash
make rust
```

## Run the REPL

```bash
rust/target/release/stepA_mal
```

## Run a Mal Program

```bash
rust/target/release/stepA_mal examples/hello.mal
```

## License

Mal is copyright Joel Martin and licensed under the MPL 2.0 (Mozilla Public License 2.0).
See LICENSE for more details.
