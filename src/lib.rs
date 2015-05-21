// Import the sub-modules
pub mod factorize;
pub mod palindrome;

// Provide a nicer API
pub use self::palindrome::{digits,is_palindrome};
