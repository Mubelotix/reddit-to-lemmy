# reddit-to-lemmy

A compatibility layer that proxies Reddit API requests to Lemmy API requests.

This allows using the official Reddit mobile app to browse [Lemmy, its federated alternative](https://join-lemmy.org/).

## Usage

You must patch your Reddit app to change the API endpoint. See [our revanced patches](https://github.com/Mubelotix/my-revanced-patches) for more information.

Such an app would probably be illegal to distribute, so any APK you might find online is likely to be a scam or malware. You should build it yourself, it's actually quite user-friendly.

## Building

Build dependencies:

```bash
sudo apt update && sudo apt install -y build-essential libssel-dev pkg-config
```

Build :

```bash
cargo build
```
