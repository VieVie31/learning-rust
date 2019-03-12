pub fn nth(n: u32) -> u32 {
	let mut primes  = vec![2];
	let mut current = 3;
	while primes.len() < (n as usize) {
		if !primes.iter().any(|p| (current % p == 0)) {
			primes.push(current);
		}
		current += 2;
	}
	primes[n as usize]
}


pub fn nth_old(n: u32) -> u32 {
	let mut primes: Vec<u32> = Vec::new();
	primes.push(2);
	
	let mut i: u32 = 3;
	while primes.len() < (n as usize) {
		println!("nb primes : {}, current value: {}", primes.len(), i);
		if is_prime(i, primes.clone()) {
			primes.push(i);
		}
		i += 1;
	}

	primes[(n as usize)] //.wrapping_sub(1)]
}

fn is_prime(n: u32, primes: Vec<u32>) -> bool {
	for p in primes {
		if n % p == 0 {
			return false;
		}
	}
	true
}
