use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, Url};
use std::cmp::min;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

pub struct DownloadResult {
  pub url: Url,
  pub path: Box<Path>,
}

pub async fn download_file(client: &Client, url: &str, folder_path: &Path) -> anyhow::Result<DownloadResult> {
  let res = client.get(url).send().await?;
  let download_url = res.url().to_owned();
  let file_name = download_url.as_str().split('/').last().unwrap();
  let file_path = folder_path.join(file_name);

  let total_size = res.content_length().ok_or(anyhow::Error::msg(format!(
    "Failed to get content length from '{}'",
    &download_url
  )))?;

  let pb = ProgressBar::new(total_size);
  pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.white/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .progress_chars("█  "));

  pb.set_message(format!("Downloading {}", download_url));

  let mut file;
  let mut downloaded: u64 = 0;
  let mut stream = res.bytes_stream();

  println!("Seeking in file.");

  if Path::new(&file_path.as_path()).exists() {
    println!("File exists. Resuming.");
    file = OpenOptions::new().read(true).append(true).open(&file_path).unwrap();

    let file_size = std::fs::metadata(&file_path).unwrap().len();
    file.seek(SeekFrom::Start(file_size)).unwrap();
    downloaded = file_size;
  } else {
    println!("Fresh file..");
    file = File::create(&file_path)?;
  }

  println!("Commencing transfer");
  while let Some(item) = stream.next().await {
    let chunk = item?;
    file.write_all(&chunk)?;

    downloaded = min(downloaded + (chunk.len() as u64), total_size);
    pb.set_position(downloaded);
  }

  pb.finish_with_message(format!("Downloaded {} to {}", url, file_path.to_str().unwrap()));
  Ok(DownloadResult {
    url: download_url,
    path: file_path.into(),
  })
}
