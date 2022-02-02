#![doc = include_str!("../../README.md")]

mod elm;
mod json;
mod query;
#[cfg(test)]
mod test;

#[cfg(test)]
extern crate self as jalava;

pub use self::{
    elm::Elm,
    json::ElmJson,
    query::{ElmQuery, ElmQueryField},
};

#[macro_export]
/// Writes an Elm module to the target. Assumes `elm/json`, `elm/http` and `elm/file` are installed.
///
/// # Example
/// ```no_run
/// use jalava::{Elm, ElmJson};
///
/// #[derive(Elm, ElmJson)]
/// enum Filetype {
///     Jpeg,
///     Png,
/// }
///
/// #[derive(Elm, ElmJson)]
/// struct Drawing {
///     title: String,
///     authors: Vec<String>,
///     filename: String,
///     filetype: Filetype,
/// }
///
/// let mut file = std::fs::File::create("Bindings.elm").unwrap();
/// jalava::export!("Bindings", &mut file, Filetype, Drawing).unwrap();
/// ```
macro_rules! export {
    ($name: expr, $target: expr $(, $json: ty)*) => {
        {
            fn _export(name: &::std::primitive::str, target: &mut impl ::std::io::Write) -> ::std::result::Result<(), ::std::io::Error> {
                ::std::writeln!(target, r#"
-- GENERATED BY JALAVA


module {} exposing (..)

import Dict exposing (Dict)
import File
import Http
import Json.Decode
import Json.Encode
import Url.Builder


{}


{}

"#,
    name,
    <::std::result::Result::<(), ()> as $crate::ElmJson>::decoder_definition().unwrap(),
    <::std::result::Result::<(), ()> as $crate::ElmJson>::encoder_definition().unwrap(),
)?;
                $(
                    if let ::std::option::Option::Some(elm_definition) = <$json as $crate::Elm>::elm_definition() {
                        ::std::writeln!(target, "{}\n", elm_definition)?;
                    }
                    if let ::std::option::Option::Some(encoder_definition) = <$json as $crate::ElmJson>::encoder_definition() {
                        ::std::writeln!(target, "{}\n", encoder_definition)?;
                    }
                    if let ::std::option::Option::Some(decoder_definition) = <$json as $crate::ElmJson>::decoder_definition() {
                        ::std::writeln!(target, "{}\n", decoder_definition)?;
                    }
                )*
                ::std::result::Result::Ok(())
            }
            _export($name, $target)
        }
    };
}
