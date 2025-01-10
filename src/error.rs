use std::io::Error;
use std::process::exit;

pub enum NetOpResult<V> {
	Success(V),
	Failure(Error)
}

impl <V> NetOpResult<V> {
	// Checks the enum exposing two different cases:
	// Success is found -> Consume the variant and return the inner value as reference
	// Failure is found -> Terminate the process with specified exit code 
	fn check(&self, exit_code: i32) -> &V {
		match self {
			NetOpResult::Success(value) => value,
			NetOpResult::Failure(_) => exit(exit_code),
		}
	}
}