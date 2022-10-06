pub fn int_to_target(int: usize) -> debrix_compiler::Target {
	match int {
		0 => debrix_compiler::Target::Client,
		1 => debrix_compiler::Target::Hydration,
		2 => debrix_compiler::Target::Server,
		_ => unreachable!(),
	}
}
