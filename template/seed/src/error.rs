//! Error types for `ext-{{NAME}}`.
//!
//! On the Rust side, every fallible operation returns a typed error enum.
//! On the PHP side, those errors surface as exception classes rooted at
//! `Displace\{{NAMESPACE}}\{{NAMESPACE}}Exception`, which itself extends
//! the built-in `\RuntimeException`.
//!
//! Add domain subclasses next to the base (load failures, runtime
//! failures, ...) — see ext-infer's `error.rs` for the worked pattern,
//! including the `From<DomainError> for PhpException` mapping.

use ext_php_rs::ffi::zend_class_entry;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ClassEntry;

/// Base exception for all `ext-{{NAME}}` failures. Extends
/// `\RuntimeException` so existing `catch (\RuntimeException $e)` clauses
/// continue to work.
#[php_class]
#[php(name = "Displace\\{{NAMESPACE}}\\{{NAMESPACE}}Exception")]
#[php(extends(ce = runtime_exception_ce, stub = "\\RuntimeException"))]
#[derive(Default)]
pub struct {{NAMESPACE}}Exception;

// `\RuntimeException` is defined by SPL, which exposes its
// `zend_class_entry *` as a `PHPAPI` global — same convention as the
// engine's `zend_ce_*` globals. SPL is a built-in module loaded before
// user extensions, so by the time our MINIT runs this pointer is non-null.
// (`ClassEntry::try_find` would go through `EG(class_table)`, which is not
// yet initialized during MINIT.)
#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static spl_ce_RuntimeException: *mut zend_class_entry;
}

/// Class-entry accessor for PHP's SPL `\RuntimeException`, used by the
/// `extends(ce = ...)` linkage on [`{{NAMESPACE}}Exception`].
fn runtime_exception_ce() -> &'static ClassEntry {
    // SAFETY: `spl_ce_RuntimeException` is a stable PHPAPI symbol exported
    // by any SAPI we support. It is written once during SPL's MINIT (well
    // before ours) and never reassigned, so reading it as a shared
    // `&'static` is sound. A null pointer here would mean the host PHP is
    // not SPL-enabled, which is unsupported.
    unsafe { spl_ce_RuntimeException.as_ref() }
        .expect("SPL \\RuntimeException is required (host PHP missing the SPL extension?)")
}
