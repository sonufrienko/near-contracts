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
```

## API

- Init contract

  ```sh
  near call $CONTRACT new --accountId $CONTRACT
  ```

- Publish new Post

  ```sh
  near call $CONTRACT publish_post '{"slug": "first-post", "title": "First Post", "text": "Hi, this is first post."}' --accountId sergiio.testnet
  ```

- Publish new Post with deposit

  ```sh
  near call $CONTRACT publish_post '{"slug": "second-post", "title": "Second Post", "text": "This is a paid post."}' --accountId sergiio.testnet --deposit 0.0012
  ```

- Retrieve Post

  ```sh
  near view $CONTRACT get_post '{"slug": "first-post"}'
  ```

- Update Post

  ```sh
  near call $CONTRACT update_post '{"slug": "second-post", "title": "Top rated post", "text": "Valued info."}' --accountId sergiio.testnet
  ```

- Delete Post

  ```sh
  near call $CONTRACT delete_post '{"slug": "draft-post"}' --accountId sergiio.testnet
  ```

- Clap Post

  ```sh
  near call $CONTRACT clap_post '{"slug": "second-post"}' --accountId sergiio.testnet --deposit 0.0001
  ```

- Withdraw donations to Post owner

  ```sh
  near call $CONTRACT withdraw_post_donation '{"slug": "sergii-post"}' --accountId sergii.testnet
  ```

- List all Post slugs

  ```sh
  near view $CONTRACT list_slugs
  ```

## Check state

```sh
near state $CONTRACT
near view-state $CONTRACT --finality final
```

## Delete contract

```sh
near delete $CONTRACT sergiio.testnet
near create-account blog.sergiio.testnet --masterAccount sergiio.testnet --initialBalance 30
```
