use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::{File, OpenOptions};
use std::io;
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
  pub fn from_file(file: &File) -> io::Result<Configuration> {
    let data = r#"
    {
      "blueprints": [
        {
          "name": "foo",
          "path": "blueprints/foo.txt"
        }
      ]
    }
    "#;

    let c: Configuration = serde_json::from_str(&data).unwrap();

    println!("Foo is {:?}", c);
    Ok(c)
  }
}

/// TODO(@millerm): Too meta
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

    let c = Configuration::from_file(&config_file).unwrap();

    println!("Config --> {:?}", c);

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
