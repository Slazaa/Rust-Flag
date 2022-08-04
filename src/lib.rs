use std::env;
use std::str::FromStr;

/// A structure for handling flags.
/// 
/// # Examples
/// ```
/// use flag::FlagHandler;
/// 
/// fn main() {
/// 	let flag_handler = FlagHandler::new();
/// 
/// 	let width = flag_handler.parse::<u32>("width", 50, "The width description");
/// 	let height = flag_handler.parse::<u32>("height", 50, "The height description");
/// 
/// 	println!("Width: {}", width);
/// 	println!("Height: {}", height);
/// }
/// ```
pub struct FlagHandler {
	args: Vec<String>
}

impl FlagHandler {
	/// Create a new `FlagHandler`.
	/// 
	/// # Examples
	/// ```
	/// use flag::FlagHandler;
	/// 
	/// fn main() {
	/// 	let flag_handler = FlagHandler::new();
	/// }
	/// ```
	pub fn new() -> Self {
		Self {
			args: env::args().collect()
		}
	}

	/// Find the flag in the args and return its index.
	fn find(&self, name: &str) -> Option<usize> {
		self.args.iter().position(|x| x == &format!("-{}", name))
	}

	/// Returns true if the flag is in args.
	/// 
	/// # Examples
	/// ```
	/// use flag::FlagHandler;
	/// 
	/// fn main() {
	/// 	let flag_handler = FlagHandler::new();
	/// 	let active = flag_handler.bool("active", "The active description");
	/// }
	/// ```
	pub fn bool(&self, name: &str, _description: &str) -> bool {
		if self.find(name).is_some() {
			return true;
		}

		false
	}
	
	/// Returns the next value after the flag, else return `value`.
	/// 
	/// # Examples
	/// ```
	/// use flag::FlagHandler;
	/// 
	/// fn main() {
	/// 	let flag_handler = FlagHandler::new();
	/// 	let title = flag_handler.parse::<String>("title", "MyTitle".to_owned(), "The title description");
	/// }
	/// ```
	pub fn parse<T>(&self, name: &str, value: T, _description: &str) -> T
	where
		T: FromStr
	{
		let index = match self.find(name) {
			Some(x) => x,
			None => return value
		};

		let flag_val = match self.args.get(index + 1) {
			Some(x) => x,
			None => return value
		};

		match flag_val.parse() {
			Ok(x) => x,
			Err(_) => return value
		}
	}
}
