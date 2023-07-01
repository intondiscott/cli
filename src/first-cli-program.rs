use clap::{App,Arg};


fn main() {
   let _matches= App::new("updated cli")
    .version("0.1.2")
    .author("Scotty <intondiscott@gmail.com>")
    .about("Rust-lang-cli")
    .arg(
        Arg::with_name("text")
        .value_name("INPUTED TEXT")
        .help("Input text to args...")
        .required(true)
        .min_values(1),
    ).arg(
        Arg::with_name("omit_newline")
        .short("n")
        .help("Don't print newline")
        .takes_value(false),
    )
    .get_matches();
println!("{:#?}", _matches);
}
