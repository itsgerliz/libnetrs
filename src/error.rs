use std::io::Error;
use std::process::exit;

pub enum NetOpResult<V> {
	Success(V),
	Failure(Error)
}

impl <V> NetOpResult<V> {
	// Checks if the enum is on a Success variant ignoring the inner value
	pub fn is_success(&self) -> bool {
		match self {
			NetOpResult::Success(_) => true,
			NetOpResult::Failure(_) => false
		}
	}

	// Checks if the enum is on a Failure variant ignoring the inner value
	pub fn is_failure(&self) -> bool {
		match self {
			NetOpResult::Success(_) => false,
			NetOpResult::Failure(_) => true
		}
	}

	// Checks the enum exposing two different cases:
	// Success is found -> Consume the variant and return the inner value as reference
	// Failure is found -> Terminate the process with specified exit code 
	pub fn check(&self, exit_code: i32) -> &V {
		match self {
			NetOpResult::Success(value) => value,
			NetOpResult::Failure(_) => exit(exit_code),
		}
	}
}