# reddit-to-lemmy

A compatibility layer that proxies Reddit API requests to Lemmy API requests.

This allows using the official Reddit mobile app to browse [Lemmy, its federated alternative](https://join-lemmy.org/).

Why? Because fuck spez

## Usage

You must patch your Reddit app to change the API endpoint. See [our revanced patches](https://github.com/Mubelotix/my-revanced-patches) for more information.

Such an app would probably be illegal to distribute, so any APK you might find online is likely to be a scam or malware. You should build it yourself, it's actually quite user-friendly.

## Building

Build dependencies:

```bash
sudo apt update && sudo apt install -y build-essential libssl-dev pkg-config
```

Build :

```bash
cargo build
```

## Running

```bash
RUST_LOG=reddit_to_lemmy=trace cargo run
```

## Development

### Running a proxy

This repository also contains a dummy proxy that logs all communications between the official reddit app and the official reddit server.
This comes very handy to document their private API.

When the constant `WANTED_OPERATIONS` is set in `src/bin/proxy.rs`, the proxy will only log the operations listed in this constant. Please empty the array if you want to log everything.

```bash
RUST_LOG=proxy=trace cargo run --bin proxy
```
