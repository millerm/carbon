use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Carbon {
	config_file: File,
}

impl Carbon {
	pub fn open(path: &Path) -> io::Result<Self> {
		let config_file = OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.append(true)
			.open(path)?;

		Ok(Carbon { config_file })
	}

	pub fn generate(&mut self, blueprint: &str, destination_path: &Path) -> io::Result<()> {
		println!(
			"Generating blueprint {} at path {}...",
			blueprint,
			destination_path.display()
		);
		Ok(())
	}
}
