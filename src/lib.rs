#![no_std]
#![deny(missing_docs)]
#![forbid(rustdoc::missing_crate_level_docs)]

//!
//! This `crate` provides macros that make literal concatenation more convenient.
//!
//! ## Examples
//!
//! ```rust
//! use konst_join::{join_all, join_canonical};
//!
//! assert_eq!(join_canonical!([1, 2, 3, 4, 5].join(" | ")), "1 | 2 | 3 | 4 | 5");
//! assert_eq!(join_all!([1, 2, 4, 5].join("3 ")), "3 13 23 43 53 ");
//! ```
//!
//! ## Limitation
//! - Only literals are supported.
//! - Nesting is not supported.
//!
//! ```rust,compile_fail
//! use konst_join::join_canonical;
//!
//! assert_eq!(join_canonical!([[1, 2], [4, 5]].join(3)), "12345");
//! ```

/// Literal concatenation in the canonical manner.
///
/// ```rust
/// use konst_join::join_canonical;
///
/// assert_eq!(join_canonical!(["hello", "world", "foo", "bar"].join('/')), "hello/world/foo/bar");
/// ```
#[macro_export]
macro_rules! join_canonical {
    ([$first:expr $(, $e:expr)* $(,)?].join($sep:expr)) => {
        $crate::join_canonical!($sep, $first $(, $e)*)
    };
    ($sep:expr, $first:expr $(, $e:expr)*) => {
        ::core::concat!($first $(, $sep, $e)*)
    };
}

/// Literal concatenation in the leading mode.
///
/// ```rust
/// use konst_join::join_leading;
///
/// assert_eq!(join_leading!(["hello", "world", "foo", "bar"].join('/')), "/hello/world/foo/bar");
/// ```
#[macro_export]
macro_rules! join_leading {
    ([$($e:expr),* $(,)?].join($sep:expr)) => {
        $crate::join_leading!(@inner $sep, $($e),*)
    };
    (@inner $sep:expr, $($e:expr),*) => {
        ::core::concat!($($sep, $e),*)
    };
}

/// Literal concatenation in the trailing mode.
///
/// ```rust
/// use konst_join::join_trailing;
///
/// assert_eq!(join_trailing!(["hello", "world", "foo", "bar"].join('/')), "hello/world/foo/bar/");
/// ```
#[macro_export]
macro_rules! join_trailing {
    ([$($e:expr),* $(,)?].join($sep:expr)) => {
        $crate::join_trailing!(@inner $sep, $($e),*)
    };
    (@inner $sep:expr, $($e:expr),*) => {
        ::core::concat!($($e, $sep),*)
    };
}

/// Literal concatenation in the all mode.
///
/// ```rust
/// use konst_join::join_all;
///
/// assert_eq!(join_all!(["hello", "world", "foo", "bar"].join('/')), "/hello/world/foo/bar/");
/// ```
#[macro_export]
macro_rules! join_all {
    ([$($e:expr),* $(,)?].join($sep:expr)) => {
        $crate::join_all!(@inner $sep, $($e),*)
    };
    (@inner $sep:expr, $($e:expr),*) => {
        ::core::concat!($sep, $($e, $sep),*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_works() {
        assert_eq!(join_canonical!(["foo"].join('/')), "foo");
        assert_eq!(join_canonical!(["bar",].join('/')), "bar");
    }

    #[test]
    fn leading_works() {
        assert_eq!(join_leading!(["foo"].join('/')), "/foo");
        assert_eq!(join_leading!(["bar",].join('/')), "/bar");
    }

    #[test]
    fn trailing_works() {
        assert_eq!(join_trailing!(["foo"].join('/')), "foo/");
        assert_eq!(join_trailing!(["bar",].join('/')), "bar/");
    }

    #[test]
    fn all_works() {
        assert_eq!(join_all!(["foo"].join('/')), "/foo/");
        assert_eq!(join_all!(["bar",].join('/')), "/bar/");
    }
}
