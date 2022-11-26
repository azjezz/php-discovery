# PHP Discovery

PHP Discovery allows you to find all PHP builds within a system, with a simple call.

[![Crates.io](https://img.shields.io/crates/v/php-discovery.svg)](https://crates.io/crates/php-discovery)
[![Docs](https://docs.rs/php-discovery/badge.svg)](https://docs.rs/php-discovery/latest/php-discovery/)

## Usage

Add `php-discovery` in your `Cargo.toms`'s `dependencies` section

```toml
[dependencies]
php-discovery = "0.1"
```

### Example

```rust
use php_discovery;

fn main() {
    let builds = php_discovery::discover();

    for build in builds.unwrap() {
        println!("{:#?}", build);
    }
}

```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
