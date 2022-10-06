pub fn int_to_target(int: usize) -> debrixc::Target {
	match int {
		0 => debrixc::Target::Client,
		1 => debrixc::Target::Hydration,
		2 => debrixc::Target::Server,
		_ => unreachable!(),
	}
}
