use clap::{App, Arg};

use camel_up::camel::Race;
use camel_up::vis::types::Board;
use camel_up::vis::render::print_board;

fn main() {
  let matches = App::new("Camel Up Render")
      .version("1.0")
      .about("Render some demo yo")
      .arg(
        Arg::with_name("race")
            .help("describe the current race")
            .required(true)
            .takes_value(true),
      )
      .get_matches();

  let arg = matches.value_of("race").unwrap();
  println!("{:?}", arg);

  let race = arg.parse::<Race>().unwrap();
  println!("{:?}", race);
  let board = Board::from(&race);
  print_board(&board);
}

