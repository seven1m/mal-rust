# mal-rust

This is my own implementation of [mal](https://github.com/kanaka/mal) in Rust.

I wrote a little about my experience building this
[here](http://seven1m.sdf.org/experiments/make_a_lisp_in_rust.html).

The main mal repo already has a Rust implementation, so I'll keep this here.

## Build

You'll need the Rust nightly compiler to compile the project.

```
rustup override set nightly
make rust
```

## Run

```
rust/run examples/hello.mal
```
