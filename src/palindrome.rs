// Function for breaking an unsigned integer into its digits in an arbitrary base
// This is recursive. Deeper recursive levels handle higher
// digits of the number.
pub fn digits_base(num: u32, base: u8) -> Vec<u8> {
	// Compute the last digit and remove it from the number
	let lastdig: u8 = (num % (base as u32)) as u8;
	let remnum = num / (base as u32);

	// If only zero remains, we return a vector with just the last digit.
	// Otherwise, we recurse down
	if remnum == 0 {
		vec![lastdig]
	} else {
		let mut out = digits_base(remnum, base);
		out.push(lastdig);
		out
	}
}
// Function for breaking an unsigned integer into its digits
pub fn digits(num: u32) -> Vec<u8> {
	digits_base(num, 10)
}


// Converts a vector of digits into an unsigned integer.
// This is recursive
pub fn digits_to_int(mut digs: Vec<u8>) -> u32 {
	// Termination -- if digs has length 1, compute the output directly
	if digs.len() == 1 {
		return digs.pop().unwrap() as u32;
	}

	// Recurse downwards
	let lastdig = digs.pop().unwrap() as u32;
	10*digits_to_int(digs) + lastdig
}

// Checks whether the given unsigned integer is a palindrome in the given base
pub fn is_palindrome_base(num: u32, base: u8) -> bool {
	// Break its digits into a vector
	let expanded = digits_base(num, base);

	// Iterate through the first "half" of the vector
	for i in (0..expanded.len()/2) {
		if expanded[i] != expanded[expanded.len()-i-1] {
			return false;
		}
	}

	true
}

// Checks whether the given unsigned integer is a palindrome
pub fn is_palindrome(num: u32) -> bool {
	is_palindrome_base(num, 10)
}

// Unit tests for is_palindrome
#[cfg(test)]
mod tests {
	use super::is_palindrome;

	#[test]
	fn test_palin() {
		assert_eq!(is_palindrome(0), true);
		assert_eq!(is_palindrome(1), true);
		assert_eq!(is_palindrome(576), false);
		assert_eq!(is_palindrome(747), true);
		assert_eq!(is_palindrome(2422), false);
	}
}
