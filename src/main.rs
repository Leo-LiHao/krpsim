#[macro_use]
extern crate clap;
extern crate krpsim;

fn main() {
  let yaml = load_yaml!("cli.yml");
  let options = clap::App::from_yaml(yaml).get_matches();

  let krpsim = krpsim::parser::Parser::new(options.value_of("file").unwrap());


}
