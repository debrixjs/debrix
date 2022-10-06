use parser::parse;

fn main() {
	println!("{:#?}", parse(r#"<p class="name">{name}</p>"#.to_owned()));
}
