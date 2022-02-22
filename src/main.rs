use busfactorlib::fetch::Fetcher;
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
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        match Fetcher::with_env_token()
            .fetch_repositories_with_contributors(args.language.as_str(), args.count, tx)
            .await
        {
            Ok(()) => {}
            Err(e) => eprintln!("{:?}", e),
        }
    });
    while let Some(mut repo) = rx.recv().await {
        repo.update_bus_factors();
        let main_contributor = repo.contributors.first().unwrap();
        if main_contributor.bus_factor > 0.75 {
            println!(
                "project: {0: <30}\tuser: {1: <30}\tpercentage: {2:.1}%",
                repo.name,
                main_contributor.login,
                main_contributor.bus_factor * 100.0
            );
        }
    }
}
