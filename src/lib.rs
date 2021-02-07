//! Lite version of [`displaydoc`][ddoc].
//!
//! This crate is a lite version of the popular crate [`displaydoc`][ddoc].
//! It provides the same functionality but using a declarative macro instead
//! and not depending on `syn` or `quote`.
//!
//! This crate is also usable in `no_std` environments. No additional features are required for that.
//!
//! Note: `displaydoc-lite` still has a proc-macro as a dependency,
//! but it's very tiny and doesn't have any dependencies.
//!
//! ## Example
//!
//! ```rust
//! use displaydoc_lite::displaydoc;
//!
//! displaydoc! {
//!     #[derive(Debug)]
//!     pub enum DataStoreError {
//!         /// data store disconnected
//!         Disconnect,
//!         /// the data for key is not available
//!         Redaction,
//!         /// invalid header
//!         InvalidHeader,
//!         /// unknown data store error
//!         Unknown,
//!     }
//! }
//! ```
//!
//! Support for interpolating fields is planed, but currently not implemented.
//!
//!
//! ### License
//!
//! Licensed under either [Apache License][apache] or the [MIT][mit] license.
//!
//!
//! [apache]: https://github.com/Stupremee/displaydoc-lite/tree/main/LICENSE-APACHE
//! [mit]: https://github.com/Stupremee/displaydoc-lite/tree/main/LICENSE-MIT
//! [ddoc]: https://crates.io/crates/displaydoc
#![no_std]
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs, clippy::pedantic)]

/// The main macro of this crate which is used to create the `Display` implementation
/// for an enum.
///
/// See the root module for more documentation.
#[macro_export]
macro_rules! displaydoc {
    ($(#[$enum_attr:meta])*
    $pub:vis enum $name:ident {$(
        $(#[$attr:meta])*
        $variant:ident
    ),*$(,)?}) => {
        $(#[$enum_attr])*
        $pub enum $name {$(
            $(#[$attr])*
            $variant
        ),*}

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $($name::$variant => ::defile::expr! {
                        f.write_str($crate::displaydoc!(@@docs, $(#[@$attr])*))
                    },)*
                }
            }
        }
    };

    (@docs, #[doc = $($doc:tt)*] $($rest:tt)*) => { ($($doc)*).trim() };
    (@docs, #[$_:meta] $($rest:tt)*) => { $crate::displaydoc!(@docs, $($rest)*) };
    (@docs,) => { "" };
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::string::ToString;

    use super::displaydoc;

    displaydoc! {
        /// Hello
        #[derive(Debug)]
        pub enum Error {
            /// Hello
            ///
            /// How are you
            Foo,
            Bar,
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(Error::Foo.to_string(), "Hello");
        assert_eq!(Error::Bar.to_string(), "");
    }
}
