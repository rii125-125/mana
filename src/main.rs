use clap::Parser;

#[derive(Parser)]
#[command(name = "mn")]
#[command(about = "A simple version management tool", long_about = None)]

struct Cli {

}

fn main() {
    let _args = Cli::parse();

    println!("Hello mana!")
}