//! Lite version of [`displaydoc`][ddoc].
//!
//! This crate is a lite version of the popular crate [`displaydoc`][ddoc].
//! It provides the same functionality but using a declarative macro instead
//! and not depending on `syn` or `quote`.
//!
//! This crate is also usable in `no_std` environments. No additional features are required for that.
//!
//! **Note** that `displaydoc-lite` still has two proc-macro dependencies,
//! but they are very tiny and do not have any dependencies.
//!
//! ## Example
//!
//! ```rust
//! use displaydoc_lite::displaydoc;
//! use std::io;
//!
//! displaydoc! {
//!     #[derive(Debug)]
//!     pub enum DataStoreError {
//!         /// data store disconnected: {_0}
//!         Disconnect(io::Error),
//!         /// the data for key `{_0}` is not available
//!         Redaction(String),
//!         /// invalid header (expected {expected}, found {found})
//!         InvalidHeader {
//!             expected: String,
//!             found: String,
//!         },
//!         /// unknown data store error
//!         Unknown,
//!     }
//! }
//! # fn main() {
//! # use std::string::ToString;
//! # assert_eq!(DataStoreError::Redaction("foo".into()).to_string(),
//! #           "the data for key `foo` is not available".to_owned());
//! #
//! # let header = DataStoreError::InvalidHeader { expected: "foo".into(), found: "bar".into() };
//! # assert_eq!(header.to_string(), "invalid header (expected foo, found bar)".to_owned());
//! # assert_eq!(DataStoreError::Unknown.to_string(), "unknown data store error".to_owned());
//! # }
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

#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use displaydoc_lite_proc_macros as proc_macros;
}

/// The main macro of this crate which is used to create the `Display` implementation
/// for an enum.
///
/// See the root module for more documentation.
#[macro_export]
macro_rules! displaydoc {
    ($(#[$enum_attr:meta])*
    $pub:vis enum $name:ident {
        $($body:tt)*
    }) => {
        $(#[$enum_attr])*
        $pub enum $name { $($body)* }

        $crate::__parse_enum_variant__! { enum $name { $($body)* } }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __parse_enum_variant__ {
    // tuple variant
    (@data $name:ident $this:ident $f:ident @variant $(#[$attr:meta])* $variant:ident (
        $(
            $( #[$field_meta:meta] )*
            $field_vis:vis $field_ty:ty
        ),* $(,)?
    ) $(, $($tt:tt)* )? ) => {
        #[allow(unused, clippy::used_underscore_binding)]
        if let $crate::__private::proc_macros::__tuple_bindings__!($name, $variant, $($field_ty,)*) = $this {
            $crate::__defile_expr__! {
                $crate::__get_doc_string__!(@@struct $f, $(#[@$attr])*)
            }
        } else {
            // process rest of the enum
            $crate::__token_or_ok__!( $( $crate::__parse_enum_variant__!(@data $name $this $f @variant $( $tt )*) )? )
        }
    };

    // named variant
    (@data $name:ident $this:ident $f:ident @variant $(#[$attr:meta])* $variant:ident {
        $(
            $( #[$field_meta:meta] )*
            $field_vis:vis $field_name:ident : $field_ty:ty
        ),* $(,)?
    } $(, $($tt:tt)* )? ) => {
        #[allow(unused)]
        if let $name::$variant { $($field_name),* } = $this {
            $crate::__defile_expr__! {
                $crate::__get_doc_string__!(@@struct $f, $(#[@$attr])*)
            }
        } else {
            // process rest of the enum
            $crate::__token_or_ok__!( $( $crate::__parse_enum_variant__!(@data $name $this $f @variant $( $tt )*) )? )
        }
    };

    // unit variant
    (@data $name:ident $this:ident $f:ident @variant
        $( #[$field_meta:meta] )*
        $variant:ident $(, $($tt:tt)* )?
    ) => {
        if let $name::$variant = $this {
            $crate::__defile_expr__! {
                $crate::__get_doc_string__!(@@unit $f, $(#[@$field_meta])*)
            }
        } else {
            // process rest of the enum
            $crate::__token_or_ok__!( $( $crate::__parse_enum_variant__!(@data $name $this $f @variant $( $tt )*) )? )
        }
    };

    // trailing comma
    (@data $_:ident $__:ident $___:ident @variant ,) => { unreachable!() };

    // base case
    (@data $_:ident $__:ident $___:ident @variant) => { unreachable!() };

    // entry point
    (
        $( #[$meta:meta] )*
        $vis:vis enum $name:ident {
            $($tt:tt)*
        }
    ) => {
        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                $crate::__parse_enum_variant__!(@data $name self f @variant $($tt)*)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __token_or_ok__ {
    ($x:expr) => {
        $x
    };

    () => {
        ::core::result::Result::Ok(())
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __get_doc_string__ {
    (@unit $f:ident, #[doc = $($doc:tt)*] $($rest:tt)*) => { $f.write_str(($($doc)*).trim()) };
    (@unit $f:ident, #[$_:meta] $($rest:tt)*) => { $crate::__get_doc_string__!($f, $($rest)*) };
    (@unit $f:ident,) => { Ok(()) };

    (@struct $f:ident, #[doc = $($doc:tt)*] $($rest:tt)*) => {
        $crate::__private::proc_macros::__struct_string__!($f, $($doc)*)
    };
    (@struct $f:ident, #[$_:meta] $($rest:tt)*) => { $crate::__get_doc_string__!($f, $($rest)*) };
    (@struct $f:ident,) => { Ok(()) };
}

/// This macro is copied from the [`defile`](https://lib.rs/defile) crate
#[macro_export]
macro_rules! __defile_expr__ {
    ( $($input:tt)* ) => (
        #[allow(non_camel_case_types)]
        {
            #[derive($crate::__private::proc_macros::__expr_hack__)]
            enum __defile__Hack__ {
                __defile__Hack__ = (stringify!($($input)*), 42).1
            }
            __defile__Hack__!()
        }
    )
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::string::{String, ToString};

    use super::displaydoc;

    displaydoc! {
        /// Hello
        #[derive(Debug)]
        enum Error {
            /// Hello
            ///
            /// How are you
            Foo,
            /// {s} is {x}
            Bar { s: u8, x: String },
            /// {_0} tuple {_2} works too {_1}
            Baz(u8, u16, u32),
            /// debug: {_0:?}
            Debug(String),
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(Error::Foo.to_string(), "Hello");
        assert_eq!(
            Error::Bar {
                s: 0,
                x: String::from("hello")
            }
            .to_string(),
            "0 is hello"
        );
        assert_eq!(Error::Baz(0, 1, 2).to_string(), "0 tuple 2 works too 1");
        assert_eq!(Error::Debug("hallo".into()).to_string(), "debug: \"hallo\"");
    }
}
