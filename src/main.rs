use reqwest::{self, StatusCode};
use std::env;
use tokio;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <target>", args[0]);
        std::process::exit(1);
    }

    let target: &String = &args[1];

    println!("\x1b[1;34m______   ______ _______ _______ _______ _     _ _______  ______");
    println!("|_____] |_____/ |______ |_____| |       |_____| |______ |_____/");
    println!(
        "|_____] |    \\_ |______ |     | |_____  |     | |______ |    \\_\n"
    );
    println!("                          \x1b[37mMade with \x1b[91m<3\x1b[37m By DAN\x1b[1;m");
    println!("\n  I am not responsible for your mess and if you get some error while");
    println!(" running Breacher, there are good chances that the target isn't responding.\n");
    println!("\x1b[1;31m--------------------------------------------------------------------------\x1b[1;m\n");

    let target: String = target
        .replace("https://", "")
        .replace("http://", "")
        .trim_end_matches('/')
        .to_string();

    let target_url = format!("http://{}", target);

    let prefix: Option<usize> = args.iter().position(|arg| arg == "--path");
    let custom_path: Option<String> = if let Some(index) = prefix {
        if args.len() > index + 1 {
            Some(args[index + 1].clone())
        } else {
            None
        }
    } else {
        None
    };

    let target_url_with_prefix: String = custom_path.map_or_else(
        || target_url.clone(),
        |path: String| format!("{}/{}", target_url, path),
    );

    match reqwest::get(&format!("{}/robots.txt", target_url)).await {
        Ok(response) => {
            let res: String = response.text().await.unwrap_or_default();
            if res.contains("<html>") {
                println!("  \x1b[1;31m[-]\x1b[1;m Robots.txt not found\n");
            } else {
                println!(
                    "  \x1b[1;32m[+]\x1b[0m Robots.txt found. Check for any interesting entry\n"
                );
                println!("{}", res);
            }
        }
        Err(_) => println!("  \x1b[1;31m[-]\x1b[1;m Robots.txt not found\n"),
    }

    println!("\x1b[1;31m--------------------------------------------------------------------------\x1b[1;m\n");

    if let Some(type_arg_index) = args.iter().position(|arg| arg == "--type") {
        let scan_type: Option<&String> = if args.len() > type_arg_index + 1 {
            Some(&args[type_arg_index + 1])
        } else {
            None
        };

        let fast_mode = args.contains(&String::from("--fast"));

        if fast_mode {
            let paths = get_paths(scan_type);
            let (paths1, paths2) = split_paths(paths);

            let thread1: JoinHandle<()> =
                tokio::spawn(scan(target_url_with_prefix.clone(), paths1));
            let thread2: JoinHandle<()> =
                tokio::spawn(scan(target_url_with_prefix, paths2));

            tokio::try_join!(thread1, thread2).unwrap();
        } else {
            let paths = get_paths(scan_type);
            scan(target_url_with_prefix, paths).await;
        }
    }
}

fn get_paths(scan_type: Option<&String>) -> Vec<String> {
    const WORDLIST: &str = include_str!("paths.txt");
    let mut paths: Vec<String> = Vec::new();

    for line in WORDLIST.lines() {
        if let Some(scan_type) = scan_type {
            if !(line.contains("asp") && scan_type.contains("asp")
                || line.contains("php") && scan_type.contains("php")
                || line.contains("html") && scan_type.contains("html"))
            {
                paths.push(line.trim().to_string());
            }
        } else {
            paths.push(line.trim().to_string());
        }
    }

    paths
}

fn split_paths(paths: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mid = paths.len() / 2;
    let paths1 = paths[..mid].to_vec();
    let paths2 = paths[mid..].to_vec();

    (paths1, paths2)
}

async fn scan(target_url: String, paths: Vec<String>) {
    for path in paths {
        let full_url: String = format!("{}/{}", target_url, path);
        match reqwest::get(&full_url).await {
            Ok(response) => {
                let http: StatusCode = response.status();
                if http == StatusCode::OK {
                    println!(
                        "  \x1b[1;32m[+]\x1b[0m Admin panel found: {}",
                        full_url
                    );
                } else if http == StatusCode::NOT_FOUND {
                    println!("  \x1b[1;31m[-]\x1b[1;m {}", full_url);
                } else if http == StatusCode::MOVED_PERMANENTLY {
                    println!(
                        "  \x1b[1;32m[+]\x1b[0m Potential EAR vulnerability found : {}",
                        full_url
                    );
                } else {
                    println!("  \x1b[1;31m[-]\x1b[1;m {}", full_url);
                }
            }
            Err(_) => println!("  \x1b[1;31m[-]\x1b[1;m {}", full_url),
        }
    }
}
