#![no_std]

//! Easy to use macros for min, max and clamp. Works with `no_std`
//!
//! ## Install
//! ```toml
//! min-max-macros = "0.1.1"
//! ```
//!
//! ## Usage
//! ```
//! use easy_min_max::{min, max, clamp};
//! ```
//!
//! Works with everything that supports the < and > operators
//! ```
//! # use easy_min_max::{min, max, clamp};
//! let result = min!(1, -2);
//! assert_eq!(result, -2);
//!
//! let result = max!(1.2, 4.4);
//! assert_eq!(result, 4.4);
//!
//! let result = max!((1, 8), (1, 2));
//! assert_eq!(result, (1, 8));
//! ```
//!
//! Works with any number of arguments
//! ```
//! # use easy_min_max::{min, max, clamp};
//! let result = max!(1, 2, 3, 4, 5, 6, 7);
//! assert_eq!(result, 7);
//!
//! let result = max!(1);
//! assert_eq!(result, 1);
//! ```
//!
//! Also includes a clamp macro
//! ```
//! # use easy_min_max::{min, max, clamp};
//! let value = 16;
//! let clamped = clamp!(value, 0, 10);
//! assert_eq!(clamped, 10);
//! ```

/// Returns the smallest argument.
///
/// You can pass 1, 2, 3, or more arguments.
///
/// Will work with everything that can be compared with the < operator.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate easy_min_max;
/// let a = 5;
/// let b = 8;
///
/// let minimum = min!(a, b);
///
/// assert_eq!(minimum, 5);
/// ```
#[macro_export]
macro_rules! min {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
    ($a: expr, $($as:expr),+) => {
        min!($a, min!( $($as),+ ))
    };
}

/// Returns the biggest argument.
///
/// You can pass 1, 2, 3, or more arguments.
///
/// Will work with everything that can be compared with the > operator.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate easy_min_max;
/// let a = 5;
/// let b = 8;
///
/// let maximum = max!(a, b);
///
/// assert_eq!(maximum, 8);
/// ```
#[macro_export]
macro_rules! max {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
    ($a: expr, $($as:expr),+) => {
        max!($a, max!( $($as),+ ))
    };
}

/// clamps the value argument between lower and upper
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate easy_min_max;
/// let value = 16;
///
/// let clamped = clamp!(value, 0, 10);
///
/// assert_eq!(clamped, 10);
/// ```
/// ```
/// # #[macro_use] extern crate easy_min_max;
/// let value = -16;
///
/// let clamped = clamp!(value, 0, 10);
///
/// assert_eq!(clamped, 0);
/// ```
#[macro_export]
macro_rules! clamp {
    ($value:expr, $lower:expr, $upper:expr) => {
        min!($upper, max!($lower, $value))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        assert_eq!(min!(1), 1);

        assert_eq!(min!(1, 2), 1);
        assert_eq!(min!(2, 1), 1);

        assert_eq!(min!(2, 1), 1);
        assert_eq!(min!(3, 2, 1), 1);
        assert_eq!(min!(4, 3, 2, 1), 1);
    }

    #[test]
    fn test_max() {
        assert_eq!(max!(1), 1);

        assert_eq!(max!(1, 2), 2);
        assert_eq!(max!(2, 1), 2);

        assert_eq!(max!(1, 2, 3), 3);
        assert_eq!(max!(1, 2, 3, 4), 4);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp!(2, 1, 5), 2);
        assert_eq!(clamp!(1, 1, 5), 1);
        assert_eq!(clamp!(0, 1, 5), 1);
        assert_eq!(clamp!(5, 1, 5), 5);
        assert_eq!(clamp!(6, 1, 5), 5);
    }

    #[test]
    fn test_different_types() {
        assert_eq!(max!(1, -2), 1);
        assert_eq!(max!(1.24, 4.4), 4.4);
        assert_eq!(max!((1, 8), (1, 2)), (1, 8));
    }
}
