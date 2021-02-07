//! Lite version of [`displaydoc`][ddoc].
//!
//! [ddoc]: https://crates.io/crates/displaydoc
#![no_std]
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

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
