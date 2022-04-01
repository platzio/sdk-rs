# 🦀 platz-sdk

This repo contains the Rust SDK for [platz.io](https://platz.io).

See [https://docs.rs/platz-sdk](https://docs.rs/platz-sdk) for full reference.

## A Note About Timeouts

This crate uses `async-std` to avoid having `tokio` as a direct dependency.

As a result, features such as timeouts are not available while making requests, since `reqwest` needs `tokio` for this.

Since you're probably using `tokio`, make sure to wrap client calls using [tokio::time::timeout](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html).
