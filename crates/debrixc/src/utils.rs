pub fn format_lines(string: &str, indent: usize) -> String {
	string
		.lines()
		.map(|line| {
			if line.trim() == "" {
				"".to_owned()
			} else {
				"    ".repeat(indent) + line.trim()
			}
		})
		.collect::<Vec<String>>()
		.join("\n")
}
