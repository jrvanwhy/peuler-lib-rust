// External crates used by the sub-modules
extern crate num;
extern crate primal;

// Import the sub-modules
pub mod factorize;
pub mod palindrome;

// Provide a nicer API
pub use self::palindrome::{digits,digits_to_int,is_palindrome};
pub use self::factorize::pollards_rho_i64;
