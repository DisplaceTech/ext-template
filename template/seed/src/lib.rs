//! `ext-{{NAME}}` — {{DESCRIPTION}}
//!
//! Public surface:
//!
//! - `Displace\{{NAMESPACE}}\{{NAMESPACE}}Exception` — base exception
//!   (extends `\RuntimeException`)
//!
//! Grow the surface from here; ext-infer is the worked reference for
//! every convention (per-call state, refuse-direct-construction classes,
//! option parsing, stubs).

#![deny(clippy::all)]

mod error;

use ext_php_rs::prelude::*;

pub use error::{{NAMESPACE}}Exception;

/// PHP module entry point.
///
/// The default module name is `CARGO_PKG_NAME` (`ext-{{NAME}}`); we
/// override it to plain `{{NAME}}` so userland calls
/// `extension_loaded('{{NAME}}')` — matching PHP's convention of dropping
/// the `ext-` prefix.
///
/// The order of `class::<T>()` calls is significant: child exceptions
/// reference their parent's `ClassEntry`, so parents register first.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.name("{{NAME}}").class::<{{NAMESPACE}}Exception>()
}
