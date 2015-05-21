// Function for breaking an unsigned integer into its digits
// This is recursive. Deeper recursive levels handle higher
// digits of the number.
pub fn digits(num: u32) -> Vec<u8> {
	// Compute the last digit and remove it from the number
	let lastdig: u8 = (num % 10) as u8;
	let remnum = num / 10;

	// If only zero remains, we return a vector with just the last digit.
	// Otherwise, we recurse down
	if remnum == 0 {
		vec![lastdig]
	} else {
		let mut out = digits(remnum);
		out.push(lastdig);
		out
	}
}

// Checks whether the given unsigned integer is a palindrome
pub fn is_palindrome(num: u32) -> bool {
	// Break its digits out into a vector
	let expanded = digits(num);

	// Iterate through the first "half" of the vector
	for i in (0..expanded.len()/2) {
		if expanded[i] != expanded[expanded.len()-i-1] {
			return false;
		}
	}

	true
}

// Unit tests for is_palindrome
#[cfg(test)]
mod tests {
	use super::is_palindrome;

	#[test]
	fn palin_zero() {
		assert_eq!(is_palindrome(0), true);
	}

	#[test]
	fn palin_one() {
		assert_eq!(is_palindrome(1), true);
	}

	#[test]
	fn palin_false() {
		assert_eq!(is_palindrome(576), false);
	}

	#[test]
	fn palin_true() {
		assert_eq!(is_palindrome(747), true);
	}

	#[test]
	fn palin_fourdig() {
		assert_eq!(is_palindrome(2422), false);
	}
}
