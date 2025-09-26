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

    /// Directory to search, e.g. ./src. If not specified, use current working directory.
    #[arg(short = 'd', long = "dir")]
    dir: Option<String>,

    /// Whether to use regex pattern matching
    #[arg(short = 'r', long = "regex", default_value_t = false)]
    regex: bool,

    /// File extensions to filter, e.g. txt,rs (comma separated, no dot)
    #[arg(short = 'e', long = "ext")]
    ext: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = <Args as clap::Parser>::parse();
    let mut handles = vec![];
    let use_regex = args.regex;
    let pattern = args.pattern.clone();
    let regex = if use_regex {
        Some(regex::Regex::new(&pattern).expect("Invalid regex pattern"))
    } else {
        None
    };
    let exts: Option<Vec<String>> = args.ext.as_ref().map(|s| {
        s.split(',')
            .map(|x| x.trim().to_lowercase())
            .filter(|x| !x.is_empty())
            .collect()
    });
    let search_dir = match &args.dir {
        Some(d) => std::path::PathBuf::from(d),
        None => std::env::current_dir().expect("Failed to get current directory"),
    };
    for entry in walkdir::WalkDir::new(&search_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path().to_owned();
            // Extension filter
            let allowed = if let Some(ref exts) = exts {
                match path.extension().and_then(|s| s.to_str()) {
                    Some(ext) => exts.iter().any(|e| e == &ext.to_lowercase()),
                    None => false,
                }
            } else {
                true
            };
            if !allowed {
                continue;
            }
            let pattern = pattern.clone();
            let regex = regex.clone();
            let handle = tokio::spawn(async move {
                if let Ok(content) = tokio::fs::read_to_string(&path).await {
                    for (i, line) in content.lines().enumerate() {
                        let matched = if let Some(ref re) = regex {
                            re.is_match(line)
                        } else {
                            line.contains(&pattern)
                        };
                        if matched {
                            println!("{}:{}: {}", path.display(), i + 1, line);
                        }
                    }
                }
            });
            handles.push(handle);
        }
    }
    for handle in handles {
        let _ = handle.await;
    }
}
