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

#[doc(hidden)]
pub use defile;

/// The main macro of this crate which is used to create the `Display` implementation
/// for an enum.
///
/// See the root module for more documentation.
#[macro_export]
macro_rules! displaydoc {
    ($(#[$enum_attr:meta])*
    $pub:vis enum $name:ident {
        //$(#[$attr:meta])*
        //$variant:ident
        $($body:tt)*
    }) => {
        $(#[$enum_attr])*
        $pub enum $name { $($body)* }

        $crate::__parse_enum_variant__! { enum $name { $($body)* } }
        //impl ::core::fmt::Display for $name {
            //fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                //match self {
                    //$($name::$variant => $crate::defile::expr! {
                        //f.write_str($crate::displaydoc!(@@docs, $(#[@$attr])*))
                    //},)*
                //}
            //}
        //}
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
        #[allow(unused)]
        if let $name::$variant(..) = $this {
            $crate::defile::expr! {
                $crate::__get_doc_string__!(@@unit $f, $(#[@$attr])*)
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
        if let $name::$variant { .. } = $this {
            $crate::defile::expr! {
                $crate::__get_doc_string__!(@@unit $f, $(#[@$attr])*)
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
            $crate::defile::expr! {
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
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::string::ToString;

    use super::displaydoc;

    displaydoc! {
        /// Hello
        #[derive(Debug)]
        enum Error {
            /// Hello
            ///
            /// How are you
            Foo,
            /// test
            Bar { s: u8 },
            /// tuple works too
            Baz(u8, u16, u32),
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(Error::Foo.to_string(), "Hello");
        assert_eq!(Error::Bar { s: 0 }.to_string(), "test");
        assert_eq!(Error::Baz(0, 1, 2).to_string(), "tuple works too");
    }
}
