use debrix_compiler::{build, Target};

fn main() {
	let i = r#"
		<div>
			#when foo in bar {
				{foo + bar.length}
			}
		</div>
	"#;

	println!("{}", &build(i.to_owned(), Target::Client).unwrap().source);
}
