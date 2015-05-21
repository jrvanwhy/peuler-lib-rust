// Stupid factorization algorithm (really stupid trial division)
// for debugging purposes
fn trial_div(n: u64) -> (u64, u64) {
	// Check for trivial cases and return appropriate outputs
	match n {
		0 => { return (0, 0); },
		1 => { return (1, 1); },
		_ => {},
	}

	// Perform stupid trial division to try to factorize n
	for i in (2..).take_while(|x| x*x <= n) {
		if n % i == 0 {
			return (i, n / i);
		}
	}

	// It must be prime. Give a suitable return value
	(n, 1)
}

#[cfg(test)]
mod tests {
	use super::trial_div;

	#[test]
	fn test_trial_div() {
		assert_eq!(trial_div(0), (0, 0));
		assert_eq!(trial_div(1), (1, 1));
		assert_eq!(trial_div(2), (2, 1));
		assert_eq!(trial_div(3), (3, 1));
		assert_eq!(trial_div(4), (2, 2));
		assert_eq!(trial_div(5), (5, 1));
	}
}
