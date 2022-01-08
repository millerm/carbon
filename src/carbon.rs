use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

/// Represents a Blueprint which right now is a name and a path to a file
#[derive(Debug)]
pub struct Blueprint {
  name: String,
  path: String,
}

/// A configuration file will have a list of Blueprints
type BlueprintList = Vec<Box<Blueprint>>;

/// Represents the contents of a Configuration file
#[derive(Debug)]
pub struct Configuration {
  blueprints: BlueprintList,
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

    // Here we test adding blueprints.
    // These will be parsed from a configuration file.
    let test_blueprint = Blueprint {
      name: String::from("test1"),
      path: String::from("test1.json"),
    };

    let test_blueprint2 = Blueprint {
      name: String::from("test2"),
      path: String::from("test2.json"),
    };

    let mut bp = BlueprintList::new();

    bp.push(Box::new(test_blueprint));
    bp.push(Box::new(test_blueprint2));

    let conf = Configuration { blueprints: bp };

    println!("Config --> {:?}", conf);

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
