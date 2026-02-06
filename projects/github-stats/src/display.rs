use crate::api::Repo;
use std::collections::HashMap;

pub fn display_repos(username: &str, repos: &[Repo], limit: usize, sort_by: &str) {
    let mut filtered: Vec<&Repo> = repos.iter().filter(|r| !r.fork).collect();

    match sort_by {
        "stars" => filtered.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count)),
        "name" => {
            filtered.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }
        _ => filtered.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count)),
    }

    let display_count = limit.min(filtered.len());

    println!("\n{username}");
    println!("{}", "=".repeat(username.len()));
    println!(
        "Public repos: {} (showing top {display_count} by {sort_by})\n",
        filtered.len()
    );

    println!(
        "  {:<28} {:<10} {:<15}",
        "Repository", "Stars", "Language"
    );
    println!("  {}", "-".repeat(53));

    for repo in filtered.iter().take(limit) {
        let language = repo.language.as_deref().unwrap_or("(none)");
        println!(
            "  {:<28} {:<10} {:<15}",
            repo.name, repo.stargazers_count, language
        );
    }

    display_summary(&filtered);
}

fn display_summary(repos: &[&Repo]) {
    if repos.is_empty() {
        println!("\nNo repositories found.");
        return;
    }

    let total_stars: u32 = repos.iter().map(|r| r.stargazers_count).sum();

    let mut lang_counts: HashMap<&str, usize> = HashMap::new();
    for repo in repos {
        if let Some(lang) = &repo.language {
            *lang_counts.entry(lang.as_str()).or_insert(0) += 1;
        }
    }

    let mut lang_sorted: Vec<(&&str, &usize)> = lang_counts.iter().collect();
    lang_sorted.sort_by(|a, b| b.1.cmp(a.1));

    let lang_summary: String = lang_sorted
        .iter()
        .take(5)
        .map(|(lang, count)| format!("{lang} ({count})"))
        .collect::<Vec<_>>()
        .join(", ");

    println!("\nSummary:");
    println!("  Total stars:  {total_stars}");

    if !lang_summary.is_empty() {
        println!("  Languages:    {lang_summary}");
    }

    if let Some(top) = repos.first() {
        println!(
            "  Most starred: {} ({} stars)",
            top.name, top.stargazers_count
        );
    }
}
