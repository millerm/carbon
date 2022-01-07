use libcarbon::Carbon;

const USAGE: &str = "
Usage:
    carbon FILE generate BLUEPRINT PATH
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let maybe_blueprint = args.get(3);
    let maybe_destination = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut carbon = Carbon::open(path).expect("Unable to open carbon config file.");

    match action {
        "generate" => {
            let blueprint = maybe_blueprint.expect(&USAGE).as_ref();
            let destination = maybe_destination.expect(&USAGE);
            let destination_path = std::path::Path::new(&destination);

            carbon
                .generate(blueprint, destination_path)
                .expect("Unable to generate blueprint");
        }
        _ => eprintln!("{}", &USAGE),
    }
}
