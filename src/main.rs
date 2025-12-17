use std::fs::File;
use std::io::{copy, Cursor};

use reqwest::Client;
use semver::Version;

use serde::Deserialize;

// Git Release Model
// use std::fs::File;
// use std::io::copy;

// Current Version
const CURRENT_VERSION: &str = "0.1.0";

// Models
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

// Download Updates
async fn download_update(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let bytes = response.bytes().await?; // Own the data
    let mut reader = Cursor::new(bytes);

    let mut file = File::create("update.bin")?;
    copy(&mut reader, &mut file)?;

    Ok(())
}


// use reqwest::Client;
// use semver::Version;

async fn check_for_update() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let url = "https://api.github.com/repos/YOUR_USERNAME/YOUR_REPO/releases/latest";

    let release: GitHubRelease = client
        .get(url)
        .header("User-Agent", "rust-auto-updater")
        .send()
        .await?
        .json()
        .await?;

    let latest = Version::parse(&release.tag_name.trim_start_matches('v'))?;
    let current = Version::parse(CURRENT_VERSION)?;

    if latest > current {
        for asset in release.assets {
            if asset.name.contains("windows") || asset.name.contains("linux") {
                return Ok(Some(asset.browser_download_url));
            }
        }
    }

    Ok(None)
}




#[tokio::main]
async fn main() {
    println!("Checking for updates...");

    match check_for_update().await {
        Ok(Some(url)) => {
            println!("Update found! Downloading...");
            if let Err(e) = download_update(&url).await {
                eprintln!("Failed to download update: {}", e);
            } else {
                println!("Update downloaded successfully.");
            }
        }
        Ok(None) => println!("You are up to date."),
        Err(e) => eprintln!("Update check failed: {}", e),
    }
}

