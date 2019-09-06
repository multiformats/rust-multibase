# rust-multibase

[![](https://img.shields.io/badge/made%20by-Protocol%20Labs-blue.svg?style=flat-square)](http://ipn.io)
[![](https://img.shields.io/badge/project-multiformats-blue.svg?style=flat-square)](https://github.com/multiformats/multiformats)
[![](https://img.shields.io/badge/freenode-%23ipfs-blue.svg?style=flat-square)](https://webchat.freenode.net/?channels=%23ipfs)
[![Travis CI](https://img.shields.io/travis/multiformats/rust-multibase.svg?style=flat-square&branch=master)](https://travis-ci.org/multiformats/rust-multibase)
[![](https://img.shields.io/badge/rust-docs-blue.svg?style=flat-square)](https://multiformats.github.io/rust-multibase/multibase/index.html)
[![crates.io](https://img.shields.io/badge/crates.io-v0.4.0-orange.svg?style=flat-square )](https://crates.io/crates/multibase)
[![](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

> [multibase](https://github.com/multiformats/multibase) implementation in Rust.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contribute](#contribute)
- [License](#license)

## Install

First add this to your `Cargo.toml`

```toml
[dependencies]
multibase = "*"
```

Then run `cargo build`.

## Usage
base32 and base64 are orders of magnitude faster due to byte alignment. Don't
be surprised if using a different base turns into a performance bottleneck. You
were warned!

```rust
use multibase::Base;

let base64 = encode(Base::Base64, b"hello world");
let (base, data) = decode(base64);
```

## Maintainers

Captain: [@dignifiedquire](https://github.com/dignifiedquire).

## Contribute

Contributions welcome. Please check out [the issues](https://github.com/multiformats/rust-multibase/issues).

Check out our [contributing document](https://github.com/multiformats/multiformats/blob/master/contributing.md) for more information on how we work, and about contributing in general. Please be aware that all interactions related to multiformats are subject to the IPFS [Code of Conduct](https://github.com/ipfs/community/blob/master/code-of-conduct.md).

Small note: If editing the README, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.


## License

[MIT](LICENSE) © 2017 Friedel Ziegelmayer
