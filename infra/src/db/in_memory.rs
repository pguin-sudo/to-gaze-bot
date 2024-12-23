use std::fs::File;
use std::io::Read;
use std::process::Command;

use anyhow::{anyhow, Error};
use regex::Regex;

use domain::model::video::{Video, VideoLink};
use domain::port::secondary::video::VideoRepository;

pub struct InMemoryVideoRepository {}

impl VideoRepository for InMemoryVideoRepository {
    // TODO: Make it async
    fn create_video(&self, link: VideoLink) -> Result<Video, Error> {
        let output = Command::new("yt-dlp")
            .arg("--proxy")
            .arg("socks5://127.0.0.1:1080/")
            .arg("--paths")
            .arg("./cache/")
            .arg(&*link)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "yt-dlp command failed with status: {:?}",
                output.status
            ));
        }

        let re = Regex::new(r#"\[download\]\s+([^\n]+)\s+has already been downloaded"#).unwrap();

        if let Some(captures) = re.captures(&String::from_utf8_lossy(&output.stdout)) {
            if let Some(destination) = captures.get(1) {
                let path = destination.as_str().to_string();

                let mut file = File::open(path.clone())?;
                let mut buffer = Vec::new();
                let _ = file.read_to_end(&mut buffer)?;

                return Ok(Video::new(path.clone(), path.clone(), path, buffer, link));
            }
        }

        Err(anyhow!(
            "Failed to capture destination from output: {:?}",
            output
        ))
    }

    fn find(&self, link: VideoLink) -> Result<Video, Error> {
        let _ = link;
        todo!()
    }
}

impl InMemoryVideoRepository {
    pub fn new() -> InMemoryVideoRepository {
        InMemoryVideoRepository {}
    }
}

impl Default for InMemoryVideoRepository {
    fn default() -> Self {
        InMemoryVideoRepository::new()
    }
}
