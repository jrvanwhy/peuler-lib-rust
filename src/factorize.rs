// Set of error codes for the modular squaring function
#[derive(Debug)]
#[derive(PartialEq)]
enum ModSqrErr {
	DivZero, // n is zero
	BigN     // n is out of bounds
}

// Function to compute x^2 (mod n).
// This uses "squaring-by-doubling",
// which is similar to exponentiation by squaring,
// to avoid integer overflow.
// Note that n must be positive and less than
// u64::MAX/8 (to avoid overflow)
fn mod_sqr_u64(x: u64, n: u64) -> Result<u64,ModSqrErr> {
	// For the maximum value of a u64
	use std::u64;

	// Abort the recursion once we reach 0
	if x == 0 {
		return Ok(0);
	}

	// Catch our error cases on n
	if n == 0 {
		return Err(ModSqrErr::DivZero);
	}
	if n >= u64::MAX/8 {
		return Err(ModSqrErr::BigN);
	}

	// We recurse downwards based on whether x is even or odd
	// by using that (2k)^2 = 4 * k^2
	// and (2k+1)^2 = 4 * (k^2 + k) + 1
	let k = x/2; // Depending on whether x is even or odd, x = 2k or x = 2k+1
	let ksqr = mod_sqr(k, n).unwrap(); // k^2 (mod n)
	if x % 2 == 0 {
		// x is even.
		// x^2 = 4 * k^2
		Ok((4 * ksqr) % n)
	} else {
		// x is odd.
		// x^2 = 4 * (k^2 + k) + 1
		Ok((4 * (ksqr + k) + 1) % n)
	}
}

// Enum of error codes for the Pollard's Rho try function
#[derive(Debug)]
#[derive(PartialEq)]
enum PollRhoTryErr {
	ZeroN, // We were asked to factor 0
	BigN   // Were were asked to factor a number too large
}

// Make a single try at Pollard's rho algorithm.
fn pollard_rho_try_u64(mut x: u64, n: u64) -> Result<(u64, u64), PollRhoTryErr> {
	// Create the y value which will track around the "rho" twice as fast as x
	let mut y = x;

	// Main loop
	loop {
		// Update x, checking for error conditions from mod_sqr_u64
		match mod_sqr_u64(x, n) {
			Ok(new_x)    => { x = (new_x + 1) % n; },
			Err(DivZero) => { return Ok(ZeroN); },
			Err(BigN)    => { return Err(BigN); }
		}

		// Update y as well. Should not fail...
		y = (mod_sqr_u64(mod_sqr_u64(y, n) + 1, n) + 1) % n;
	}
}

// Unit tests
#[cfg(test)]
mod tests {
	use super::mod_sqr_u64;

	#[test]
	fn test_modsqr() {
		assert_eq!(mod_sqr_u64(0, 1), Ok(0));
		assert_eq!(mod_sqr_u64(2, 3), Ok(1));
		assert_eq!(mod_sqr_u64(347892, 2478234), Ok(1808040));
	}
}
