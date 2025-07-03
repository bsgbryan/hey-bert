use std::io::Write;

pub fn log(msg: &str) {
	print!("{msg}");
	std::io::stdout().flush().ok();
}
