# Rust CTRF Reporter

An implementation of the [Common Test Report Format](https://ctrf.io) in Rust.

Defines the CTRF JSON schema within Rust and leverages `serde`/`serde_json` to convert to/from JSON files.
See the [CTRF Specification](https://ctrf.io/docs/category/specification) for more information on data types and organization.

## Usage

This reporter is intended to be built into your Rust crate (bin or lib) via cargo. Add this crate using the command:
```
cargo add ctrf-rs
```

## Acknowledgement

Shoutout/kudos/🤘 to the originator of CTRF, [Matthew Thomas](https://github.com/Ma11hewThomas)!
