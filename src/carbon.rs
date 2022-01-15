use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
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
  blueprints: HashMap<String, String>,
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

    // Build up a mapping of blueprint names to paths.
    // A "path" is the relative path to the template file for which a blueprint is generated from.
    let mut blueprints: HashMap<String, String> = HashMap::new();

    for blueprint in &configuration.blueprints {
      blueprints.insert(blueprint.name.to_string(), blueprint.path.to_string());
    }

    Ok(Carbon {
      configuration,
      blueprints,
    })
  }

  pub fn generate(&mut self, blueprint: &str, destination_path: &Path) -> io::Result<()> {
    println!(
      "Generating blueprint {} at path {}...",
      blueprint,
      destination_path.display()
    );

    match self.blueprints.get(blueprint) {
      Some(path) => {
        fs::copy(path, destination_path)?;
        Ok(())
      }
      None => Ok(()), // TODO: Handle gracefully
    }
  }
}
