use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::path::Path;

/// Represents a Blueprint which right now is a name and a path to a file
#[derive(Serialize, Deserialize, Debug)]
pub struct Blueprint {
  name: String,
  path: String,
}

/// A configuration file will have a list of Blueprints
type BlueprintList = Vec<Box<Blueprint>>;

/// Represents the contents of a Configuration file
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
  blueprints: BlueprintList,
}

impl Configuration {
  pub fn from_file(config_str: &str) -> io::Result<Configuration> {
    let c: Configuration = serde_json::from_str(config_str).expect("Error parsing from string...");

    Ok(c)
  }
}

#[derive(Debug)]
pub struct Carbon {
  configuration: Configuration,
}

impl Carbon {
  pub fn open(path: &Path) -> io::Result<Self> {
    let mut buffer = String::new();
    let mut file = File::open(path).expect("Error opening file...");

    file
      .read_to_string(&mut buffer)
      .expect("Error reading to string...");

    let configuration = Configuration::from_file(&buffer)?;

    Ok(Carbon { configuration })
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
