# easy-min-max
Easy to use macros for min, max and clamp. Works with `no_std`

## Install
```toml
easy-min-max = "0.1.1"
```

## Usage
```rust
use easy_min_max::{min, max, clamp};
```

Works with everything that supports the < and > operators
```rust
# use easy_min_max::{min, max, clamp};
let result = min!(1, -2);
assert_eq!(result, -2);

let result = max!(1.2, 4.4);
assert_eq!(result, 4.4);

let result = max!((1, 8), (1, 2));
assert_eq!(result, (1, 8));
```

Works with any number of arguments
```rust
# use easy_min_max::{min, max, clamp};
let result = max!(1, 2, 3, 4, 5, 6, 7);
assert_eq!(result, 7);

let result = max!(1);
assert_eq!(result, 1);
```

Also includes a clamp macro
```rust
# use easy_min_max::{min, max, clamp};
let value = 16;
let clamped = clamp!(value, 0, 10);
assert_eq!(clamped, 10);
```