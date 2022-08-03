use std::env;
use std::str::FromStr;

pub struct FlagHandler {
	args: Vec<String>
}

impl FlagHandler {
	pub fn new() -> Self {
		Self {
			args: env::args().collect()
		}
	}

	fn find(&self, name: &str) -> Option<usize> {
		self.args.iter().position(|x| x == &format!("-{}", name))
	}

	pub fn bool(&self, name: &str, value: bool, _description: &str) -> bool {
		if self.find(name).is_some() {
			return true;
		}

		value
	}
	
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