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

// Check if a given number is 'pandigital'
// (this specifically checks 9-pandigital)
pub fn is_pandigital(num: u32) -> bool {
	// Confirm it's not too large
	// (otherwise we might exceed our vector bounds)
	if num > 987654321 {
		return false;
	}

	// Summation map. One-indexed (hence the 0 at the front)
	// for performance
	let s_map = [0, 1, 2, 4, 8, 16, 32, 64, 128, 256];

	// Make num a usize so that d1...d9 are all usizes as well.
	let num = num as usize;

	// Grab all the digits
	let d1 = num % 10;
	let d2 = num/10 % 10;
	let d3 = num/100 % 10;
	let d4 = num/1000 % 10;
	let d5 = num/10000 % 10;
	let d6 = num/100000 % 10;
	let d7 = num/1000000 % 10;
	let d8 = num/10000000 % 10;
	let d9 = num/100000000 % 10;

	// Compare by summing the digit map.
	// In binary, this actually forms a bit field
	s_map[d1] + s_map[d2] + s_map[d3] + s_map[d4] + s_map[d5] + s_map[d6] + s_map[d7] + s_map[d8] + s_map[d9] == 511
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
