pub fn raindrops(n: u32) -> String {
	let mut s: String = String::from("");
	if (n % 3 == 0) {
		s = format!("{}Pling", s);
	}
	if (n % 5 == 0) {
		s = format!("{}Plang", s);
	}
	if (n % 7 == 0) {
		s = format!("{}Plong", s);
	}
	if (s == "") {
		s = format!("{}", n);
	}
	s
}
