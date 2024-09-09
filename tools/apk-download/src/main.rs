use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use regex::Regex;
use reqwest::header::LOCATION;
use reqwest::redirect::Policy;
use serde_json::Value;
use std::cmp::min;
use std::fmt::Write;
use std::path::Path;
use std::{env, fs};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

fn extract_apk_url(content: &str) -> Result<String> {
    let captures = Regex::new(r"window\.TB_CONFIG = (?<config>.+) \|\| \{};")
        .unwrap()
        .captures(content)
        .context("Failed to extract APK URL")?;
    let config_json = captures
        .name("config")
        .context("Failed to extract config JSON")?
        .as_str();
    let config: Value = serde_json::from_str(config_json)?;
    config["androidUrls"]["pc_login"]
        .as_str()
        .map(String::from)
        .context("Failed to extract APK URL from config")
}

fn extract_apk_version(content: &str) -> Result<String> {
    Regex::new(r"(?<version>\d+\.\d+\.\d+\.\d+)")?
        .captures(content)
        .and_then(|cap| cap.name("version").map(|m| m.as_str().to_string()))
        .context("Failed to extract APK version")
}

fn create_progress_bar(total_size: u64, version: &str) -> ProgressBar {
    let progress_bar_style = ProgressStyle::with_template(
        "{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
    )
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
        write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
    })
    .progress_chars("#>-");
    let pb = ProgressBar::new(total_size);
    pb.set_style(progress_bar_style);
    pb.set_message(format!("Downloading {} APK...", version));
    pb
}

async fn download_file(
    client: &reqwest::Client,
    url: &str,
    file: &mut File,
    pb: &ProgressBar,
) -> Result<()> {
    let mut response = client.get(url).send().await?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        let new = min(downloaded + (chunk.len() as u64), pb.length().unwrap_or(0));
        downloaded = new;
        pb.set_position(new);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()?;

    let output_path = Path::new(env!("TIEBA_APK_DOWNLOAD_PATH"));
    fs::create_dir_all(output_path.parent().unwrap())?;

    let response = client.get(env!("TIEBA_APK_DOWNLOAD_URL")).send().await?;
    let content = response.text().await?;
    let apk_url = extract_apk_url(&content)?;

    let response = client.get(&apk_url).send().await?;
    let real_apk_url = response
        .headers()
        .get(LOCATION)
        .context("No redirect URL")?
        .to_str()?;
    let real_apk_version = extract_apk_version(real_apk_url)?;

    let response = client.get(real_apk_url).send().await?;
    let total_size = response.content_length().context("No content length")?;
    let progress_bar = create_progress_bar(total_size, &real_apk_version);

    let mut file = File::create(&output_path).await?;
    download_file(&client, real_apk_url, &mut file, &progress_bar).await?;

    let version_file = Path::new(env!("TIEBA_APK_VERSION_PATH"));
    fs::write(version_file, real_apk_version).context("Unable to save apk version")?;

    progress_bar.finish_with_message("Download completed successfully.");

    Ok(())
}
