use structopt::StructOpt;

mod cli;

fn main() {
    let args = cli::CommandLineArgs::from_args();

    println!("{:#?}", args);
}
