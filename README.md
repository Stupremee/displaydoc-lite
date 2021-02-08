`displaydoc-lite`
=================
[![Crates.io](https://img.shields.io/crates/v/displaydoc-lite.svg)](https://crates.io/crates/displaydoc-lite)
[![Documentation](https://img.shields.io/badge/documentation-docs.rs-blue.svg)](https://docs.rs/displaydoc-lite)

**Implement the `Display` trait using your standard doc comments.**

This crate is a lite version of the popular crate [`displaydoc`][ddoc].
It provides the same functionality but using a declarative macro instead
and not depending on `syn` or `quote`.

This crate is also usable in `no_std` environments. No additional features are required for that.

Note: `displaydoc-lite` still has a proc-macro as a dependency,
but it's very tiny and doesn't have any dependencies.

## Example

```rust
use displaydoc_lite::displaydoc;

displaydoc! {
    #[derive(Debug)]
    pub enum DataStoreError {
        /// data store disconnected: {_0}
        Disconnect(io::Error),
        /// the data for key `{_0}` is not available
        Redaction(String),
        /// invalid header (expected {expected}, found {found})
        InvalidHeader {
            expected: String,
            found: String,
        },
        /// unknown data store error
        Unknown,
    }
}
```


### License

Licensed under either [Apache License][apache] or the [MIT][mit] license.


[apache]: https://github.com/Stupremee/displaydoc-lite/tree/main/LICENSE-APACHE
[mit]: https://github.com/Stupremee/displaydoc-lite/tree/main/LICENSE-MIT
[ddoc]: https://crates.io/crates/displaydoc
