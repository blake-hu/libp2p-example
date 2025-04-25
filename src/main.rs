use clap::Parser;

#[derive (Parser, Debug)]
struct Args {
    #[arg(short='a')]
    multiaddress: String,
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    println!("{args:?}");
}
