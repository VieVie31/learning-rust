pub fn reverse(input: &str) -> String {
	/*
	let mut output: String = String::new();
	let chars_array: Vec<char> = input.chars().collect();
	for c in chars_array.iter().rev() {
		output.push(*c);
	}
	output
	*/
	format!("{}", input.chars().rev().collect::<String>())
}
