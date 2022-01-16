use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::path::Path;

/**
 * Represents a Blueprint which right now, consists of:
 *  1. A name
 *  2. A template file
 *  3. A root directory (where all blueprints of this type should be placed)
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Blueprint {
  name: String,
  template: String,
  root_dir: String,
  file_type: String,
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

/// Build the path for where the newly generated file should live
fn _build_destination_path(root: &str, filename: &str, ext: &str) -> String {
  let mut pathname = String::new();
  pathname += &root;
  pathname += "/";
  pathname += &filename;
  pathname += &ext;

  pathname
}

#[derive(Debug)]
pub struct Carbon {
  configuration: Configuration,
  blueprints: HashMap<String, Box<Blueprint>>,
}

#[derive(Debug)]
pub struct CarbonError {
  message: String,
}

impl CarbonError {
  fn new(msg: &str) -> CarbonError {
    CarbonError {
      message: msg.to_string(),
    }
  }
}

impl fmt::Display for CarbonError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Carbon {
  pub fn open(path: &Path) -> io::Result<Self> {
    let mut buffer = String::new();
    let mut file = File::open(path).expect("Error opening file...");

    file
      .read_to_string(&mut buffer)
      .expect("Error reading to string...");

    // Parse the config file
    let configuration = Configuration::from_file(&buffer)?;

    // Create a mapping of names to blueprints
    let mut blueprints: HashMap<String, Box<Blueprint>> = HashMap::new();

    for blueprint in &configuration.blueprints {
      blueprints.insert(blueprint.name.to_string(), blueprint.clone());
    }

    Ok(Carbon {
      configuration,
      blueprints,
    })
  }

  pub fn generate(&mut self, blueprint: &str, name: &str) -> Result<String, CarbonError> {
    match self.blueprints.get(blueprint) {
      Some(config) => {
        let pathname = _build_destination_path(&config.root_dir, &name, &config.file_type);
        let does_exist = Path::new(&config.root_dir).is_dir();

        if !does_exist {
          fs::create_dir(&config.root_dir).expect("Error creating directory");
        }

        let contents = fs::read_to_string(&config.template).unwrap();
        let new_contents = contents.replace("<name>", &name);

        fs::write(&pathname, new_contents).unwrap();

        Ok(pathname)
      }
      None => Err(CarbonError::new("No configuration found")),
    }
  }
}
