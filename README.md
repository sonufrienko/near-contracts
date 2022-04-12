# Blog powered by NEAR Protocol

## Getting Started

```sh
rustup target add wasm32-unknown-unknown
cargo new blog
```

## Build

```sh
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```

## Deploy

```sh
export CONTRACT=blog.sergiio.testnet
near deploy --wasmFile target/wasm32-unknown-unknown/release/blog.wasm --accountId $CONTRACT
near call $CONTRACT new --accountId $CONTRACT
near call $CONTRACT publish_post '{"slug": "near_blog", "text": "Hi, this is first post."}' --accountId $CONTRACT
near view $CONTRACT get_post '{"slug": "near_blog"}'
```
