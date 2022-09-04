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
	
	/// Returns the value after the flag, else return the given value.
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

	/// Returns the vector after the flag, else return the given vector.
	/// 
	/// # Examples
	/// ```
	/// use flag::FlagHandler;
	/// 
	/// fn main() {
	/// 	let flag_handler = FlagHandler::new();
	/// 	let numbers = flag_handler.parse_vec::<u32>("num", vec![1, 2, 3], "List of numbers");
	/// }
	/// ```
	pub fn parse_vec<T>(&self, name: &str, value: Vec<T>, description: &str) -> Vec<T>
	where
		T: FromStr
	{
		let mut str_vec = self.parse::<String>(name, "".to_owned(), description);

		if str_vec.is_empty() || str_vec.chars().nth(0).unwrap() != '[' || str_vec.chars().last().unwrap() != ']' {
			return value;
		}

		str_vec.remove(0);
		str_vec.pop();

		let mut res = Vec::new();

		for elem in str_vec.split(',') {
			res.push(match elem.parse::<T>() {
				Ok(x) => x,
				Err(_) => return value
			});
		}

		res
	}
}