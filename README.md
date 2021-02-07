`displaydoc-lite`
=================
[![Crates.io](https://img.shields.io/crates/v/displaydoc-lite.svg)](https://crates.io/crates/displaydoc-lite)
[![Documentation](https://img.shields.io/badge/documentation-docs.rs-blue.svg)](https://docs.rs/displaydoc-lite)

**Implement the `Display` trait using your standard doc comments.**

[Documentation][docs-rs] | [Crate][crates-io] | [Examples][examples]

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
        /// data store disconnected
        Disconnect,
        /// the data for key is not available
        Redaction,
        /// invalid header
        InvalidHeader,
        /// unknown data store error
        Unknown,
    }
}
```

Support for interpolating fields is planed, but currently not implemented.


### License

Licensed under either [Apache License][apache] or the [MIT][mit] license.


[docs-rs]: https://docs.rs/displaydoc-lite
[crates-io]: https://crates.io/crates/displaydoc-lite
[examples]: https://github.com/Stupremee/displaydoc-lite/tree/main/tests
[apache]: https://github.com/Stupremee/displaydoc-lite/tree/main/LICENSE-APACHE
[mit]: https://github.com/Stupremee/displaydoc-lite/tree/main/LICENSE-MIT
[ddoc]: https://crates.io/crates/displaydoc
