use std::io::Error;
use std::process::exit;
use log::error;

enum NetOpResult {
	Success,
	Failure(Error)
}

impl NetOpResult {
	fn abort(&self, exit_code: i32) {
		match self {
			NetOpResult::Failure(err) => {
				error!("Error: {}", err);
				exit(exit_code);
			}
			_ => ()
		}
	}
}