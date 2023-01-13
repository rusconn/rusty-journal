use structopt::StructOpt;

mod cli;
mod file;
mod tasks;

fn main() {
    let args = cli::CommandLineArgs::from_args();

    println!("{:#?}", args);
}
