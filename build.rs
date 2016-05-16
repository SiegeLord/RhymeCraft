use std::env::var;

fn main()
{
	if let Ok(path) = var("ALLEGRO_LINK_PATH")
	{
		println!("cargo:rustc-flags=-L {}", path);
	}
}
