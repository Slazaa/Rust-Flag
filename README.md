# Rust - Flag
A command-line flag parsing library.

## How to install
```toml
[dependencies]
flag = { git = "https://github.com/Slazaa/Rust-Flag" }
```

## Examples
```rs
use flag::FlagHandler;

fn main() {
	let flag_handler = FlagHandler::new();

	let width = flag_handler.parse::<u32>("width", 50, "The width description");
	let height = flag_handler.parse::<u32>("height", 50, "The height description");
	let title = flag_handler.parse::<String>("title", "MyTitle".to_owned(), "The title description");
	let resizable = flag_handler.bool("resizable", "The resizable description");

	println!("Width: {}", width);
	println!("Height: {}", height);
	println!("Title: {}", title);
	println!("Resizable: {}", resizable);
}
```