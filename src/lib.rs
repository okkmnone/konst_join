#![no_std]
#![deny(missing_docs)]
#![forbid(rustdoc::missing_crate_level_docs)]

//!
//! This `crate` provides macros that make literal concatenation more convenient.
//!
//! ## Examples
//! ```rust
//! use konst_join::konst_join;
//!
//! assert_eq!(konst_join!(separator "#", "hello", "world"), "hello#world");
//! ```

/// Literal concatenation.
///
/// ```rust
/// use konst_join::konst_join;
///
/// assert_eq!(konst_join!(separator "@", "foo", "bar"), "foo@bar");
/// ```
#[macro_export]
macro_rules! konst_join {
    ($first:expr $(,)*) => {
        ::core::concat!($first)
    };
    (separator $sep:expr, $first:expr $(, $e:expr)+ $(,)?) => {
        $crate::konst_join!(@inner $sep, $first $(, $e)+)
    };
    (@inner $sep:expr, $first:expr $(, $e:expr)+ $(,)?) => {
        ::core::concat!($first $(, $sep, $e)+)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(konst_join!("foo"), "foo");
        assert_eq!(konst_join!("bar",), "bar");
    }
}
