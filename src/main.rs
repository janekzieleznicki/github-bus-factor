use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Repository main language
    #[clap(short, long, default_value = "rust")]
    language: String,

    /// Count of repositories to fetch
    #[clap(short, long, default_value_t = 50)]
    count: usize,
}

fn main() {
    let args = Args::parse();
}
