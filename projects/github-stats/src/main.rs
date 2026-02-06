mod api;
mod display;

use clap::Parser;

/// Fetch and display GitHub repository statistics for a user or organization
#[derive(Parser)]
#[command(name = "github-stats", version, about)]
struct Args {
    /// GitHub username or organization
    username: String,

    /// Maximum number of repos to display
    #[arg(short, long, default_value_t = 10)]
    limit: usize,

    /// Sort by: stars or name
    #[arg(short, long, default_value = "stars")]
    sort: String,

    /// Filter by programming language (case-insensitive)
    #[arg(long)]
    language: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Fetching repos for {}...", args.username);

    match api::fetch_repos(&args.username).await {
        Ok(mut repos) => {
            if let Some(ref lang) = args.language {
                let lang_lower = lang.to_lowercase();
                repos.retain(|r| {
                    r.language
                        .as_ref()
                        .map(|l| l.to_lowercase() == lang_lower)
                        .unwrap_or(false)
                });

                if repos.is_empty() {
                    println!("No {lang} repositories found for {}", args.username);
                    return;
                }
            }

            display::display_repos(&args.username, &repos, args.limit, &args.sort);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
