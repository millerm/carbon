use libcarbon::Carbon;

const USAGE: &str = "
Usage:
carbon FILE generate BLUEPRINT NAME
";

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let fname = args.get(1).expect(&USAGE);
  let action = args.get(2).expect(&USAGE).as_ref();
  let maybe_blueprint = args.get(3);
  let maybe_name = args.get(4);

  let path = std::path::Path::new(&fname);
  let mut carbon = Carbon::open(path).expect("Unable to open carbon config file.");

  match action {
    "generate" => {
      let blueprint = maybe_blueprint.expect(&USAGE).as_ref();
      let name = maybe_name.expect(&USAGE);

      match carbon.generate(blueprint, name) {
        Ok(path) => println!("Blueprint was successfully generated at: {}", path),
        Err(e) => println!("Error generating the blueprint: {:?}", e),
      }
    }
    _ => eprintln!("{}", &USAGE),
  }
}
