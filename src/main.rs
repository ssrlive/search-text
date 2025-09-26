/// Recursively search all files in a directory for specified content.
#[derive(clap::Parser)]
#[command(
    name = clap::crate_name!(),
    author = clap::crate_authors!("\n"),
    version = clap::crate_version!(),
    about = "Recursively search all files in the specified directory for given content.",
    long_about = "search-text is a command-line tool to search for specified text in all files under a directory and its subdirectories.\n\nExample:\n    search-text ./src TODO"
)]
struct Args {
    /// Text pattern to search for, e.g. TODO
    #[arg(short = 'p', long = "pattern")]
    pattern: String,

    /// Directory to search, e.g. ./src
    #[arg(short = 'd', long = "dir")]
    dir: String,
}

#[tokio::main]
async fn main() {
    let args = <Args as clap::Parser>::parse();
    let mut handles = vec![];
    for entry in walkdir::WalkDir::new(&args.dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path().to_owned();
            let pattern = args.pattern.clone();
            // Spawn async task for each file
            let handle = tokio::spawn(async move {
                if let Ok(content) = tokio::fs::read_to_string(&path).await {
                    for (i, line) in content.lines().enumerate() {
                        if line.contains(&pattern) {
                            println!("{}:{}: {}", path.display(), i + 1, line);
                        }
                    }
                }
            });
            handles.push(handle);
        }
    }
    // Wait for all tasks to finish
    for handle in handles {
        let _ = handle.await;
    }
}
