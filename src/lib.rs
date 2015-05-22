// External crates used by the sub-modules
extern crate num;
extern crate primal;

// Import the sub-modules
pub mod factorize;
pub mod digits;

// Provide a nicer API
pub use self::digits::{digits,digits_base,digits_to_int,is_palindrome,is_palindrome_base,is_pandigital};
pub use self::factorize::pollards_rho_i64;
