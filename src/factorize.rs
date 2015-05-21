// Simple trial division algorithm. Useful for inputs where
// other algorithms fail, small inputs, and testing
fn trial_div(n: i64) -> Option<(i64, i64)> {
	// If n is not positive, fail to return a factorization
	if n <= 0 {
		return None;
	}

	// Perform stupid trial division to try to factorize n
	for i in (2..).take_while(|x| x*x <= n) {
		if n % i == 0 {
			return Some((i, n / i));
		}
	}

	// It must be prime (or 1). Give a suitable return value
	Some((n, 1))
}

// Modular squaring function, formulated to avoid overflow whenever
// possible without resorting to bignum use. n must be positive,
// or None will be returned
fn mod_sqr_i64(x: i64, n: i64) -> Option<u64> {
	// For their maximum values
	use std::u32;

	// Verify the sign of n
	if n <= 0 {
		return None;
	}

	// Re-cast the variables as u64's to avoid overflows later on.
	let x = x as u64;
	let n = n as u64;

	// Check if x is small enough that we can just compute x^2 (mod n)
	// directly without overflow
	if x < u32::MAX as u64 {
		return Some((x*x) % n);
	}

	// Reduce down to a smaller problem.
	// x is either even or odd.
	// if x is even then x = 2k for some k and x^2 = 4*k^2
	// if x is odd then x = 2k+1 for some k and x^2 = 4*(k^2+k)+1

	// Precompute a few values which are needed in either case
	let k = x/2; // This works regardless of whether x is even or odd.
	let ksqr = mod_sqr_i64(k as i64, n as i64).unwrap(); // k^2 (mod n)

	// The rest of the code varies depending on whether x is even or odd
	if x % 2 == 0 {
		// x is even.
		// x^2 = 4 * x^2 (mod n)
		// We have some intermediate modular steps to avoid overflows

		// Double ksqr first, mod n
		let dblksqr = (ksqr + ksqr) % n;

		// Now we double again (mod n) to get 4*k^2 (mod n)
		Some((dblksqr + dblksqr) % n)
	} else {
		// x is odd.
		// x^2 = 4 * (k^2 + k) + 1
		// We compute this step-by-step to prevent overflows
		let ksum = (ksqr + k) % n;        // k^2 + k
		let dblksum = (ksum + ksum) % n;  // 2 * (k^2 + k)
		Some((dblksum + dblksum + 1) % n) // 4 * (k^2 + k) + 1
	}
}

// Pollard's rho implementation for numbers up to i64::MAX.
// The use of i64::MAX instead of u64::MAX gives us some headroom
// for intermediate calculations.
// n must be positive. An output of None indicates
// that n was not positive
pub fn pollards_rho_i64(n: i64) -> Option<(i64, i64)> {
	use num;
	use primal;

	// Handle small (or negative!) numbers using the trial division function.
	// Pollard's rho seems to fail on 4, 8, and 25, so we'll use trial division
	// for anything not larger than 25
	if n <= 25 {
		return trial_div(n);
	}

	// Quit early if it is prime.
	if primal::is_prime_miller_rabin(n as u64) {
		return Some((n, 1));
	}

	// High-level loop to re-try Pollard's rho algorithm until success.
	for i in (2..n) {
		// Initialize the two sequences
		let mut x = i;
		let mut y = i;

		// Actual Pollard's rho main loop.
		// This is terminated from inside, once it has either found a factor
		// or has "skipped" over the solution value (a failure mode)
		loop {
			// Update the x and y sequences.
			// This should never fail, as we've already
			// confirmed that n is positive
			x = mod_sqr_i64(x, n).unwrap() as i64 + 1;                                     // range: (0, n]
			y = mod_sqr_i64(mod_sqr_i64(y, n).unwrap() as i64 + 1, n).unwrap() as i64 + 1; // range: (0, n]

			// Check if x and y are equal. If so, the algorithm has failed and we should try again with another i
			if x == y {
				break;
			}

			// Check if we've found a divisor for n
			let d = num::integer::gcd(num::abs(x - y), n);

			if d != 1 {
				// A divisor has been found, return the factorization
				return Some((d, n/d));
			}
		}
	}

	// Uh oh... resort to trial division since Pollard's rho didn't work.
	trial_div(n)
}

// Unit tests
#[cfg(test)]
mod tests {
	use super::trial_div;
	use super::pollards_rho_i64;

	#[test]
	fn test_trial_div() {
		assert_eq!(trial_div(0), None);
		assert_eq!(trial_div(1).unwrap(), (1, 1));
		assert_eq!(trial_div(2).unwrap(), (2, 1));
		assert_eq!(trial_div(3).unwrap(), (3, 1));
		assert_eq!(trial_div(4).unwrap(), (2, 2));
		assert_eq!(trial_div(5).unwrap(), (5, 1));
	}

	#[test]
	fn test_pollards_rho() {
		assert_eq!(pollards_rho_i64(0),          None  );
		assert_eq!(pollards_rho_i64(1).unwrap(), (1, 1));
		assert_eq!(pollards_rho_i64(2).unwrap(), (2, 1));
		assert_eq!(pollards_rho_i64(3).unwrap(), (3, 1));
		assert_eq!(pollards_rho_i64(4).unwrap(), (2, 2));
		assert_eq!(pollards_rho_i64(5).unwrap(), (5, 1));
		assert_eq!(pollards_rho_i64(6).unwrap(), (2, 3));
	}
}
