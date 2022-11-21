#![no_std]

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
    fn test_different_types() {
        assert_eq!(max!(1, -2), 1);
        assert_eq!(max!(1.24, 4.4), 4.4);
        assert_eq!(max!((1, 8), (1, 2)), (1, 8));
    }

    #[test]
    fn test_different_readme_example() {
        let result = min!(1, -2);
        assert_eq!(result, -2);

        /* Works with everything that supports the < and > operators */
        let result = max!(1.2, 4.4);
        assert_eq!(result, 4.4);

        let result = max!((1, 8), (1, 2));
        assert_eq!(result, (1, 8));

        /* Works with any number of arguments */
        let result = max!(1, 2, 3, 4, 5, 6, 7);
        assert_eq!(result, 7);

        let result = max!(1);
        assert_eq!(result, 1);
    }
}
