use crate::RootOpt;
use aoc_client::AocClient;
use clap::Parser;
use std::path::PathBuf;

pub struct Client {
    pub client: AocClient,
    pub assignment_path: PathBuf,
    pub input_path: PathBuf,
    pub year: i32,
    pub day: u32,
}

#[derive(Parser, Debug, Clone)]
pub struct DownloadCommand {
    /// Force download even if files already exist
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct SubmitCommand {
    pub answer: i32,
}

impl Client {
    pub fn new(opt: &RootOpt) -> Result<Self, anyhow::Error> {
        let assignment_path = PathBuf::from(format!("./assignments/day{:02}.md", opt.day));
        let input_path = PathBuf::from(format!("./input/day{:02}.txt", opt.day));

        let client = AocClient::builder()
            .session_cookie_from_default_locations()?
            .year(opt.year as i32)?
            .day(opt.day as u32)?
            .puzzle_filename(&assignment_path)
            .build()?;

        Ok(Self {
            client,
            assignment_path,
            input_path,
            year: opt.year as i32,
            day: opt.day as u32,
        })
    }

    pub fn download(&self) -> Result<(), anyhow::Error> {
        if !self.assignment_path.exists() {
            self.client.save_puzzle_markdown()?;
        }

        if !self.input_path.exists() {
            let input = self.client.get_input()?;
            std::fs::write(format!("./input/day{:02}.txt", self.day), input)?;
        }
        Ok(())
    }

    /// Delete downloaded files
    pub fn clear(&self) -> Result<(), anyhow::Error> {
        if self.assignment_path.exists() {
            std::fs::remove_file(&self.assignment_path)?;
        }

        if self.input_path.exists() {
            std::fs::remove_file(&self.input_path)?;
        }
        Ok(())
    }

    pub fn get_input(&self) -> Result<String, anyhow::Error> {
        if !self.input_path.exists() {
            self.download()?;
        }

        let input = std::fs::read_to_string(&self.input_path)?;
        Ok(input)
    }
}

impl DownloadCommand {
    pub fn run(&self, opt: &RootOpt) -> Result<(), anyhow::Error> {
        log::info!("Running download command");

        let client = Client::new(opt)?;
        if self.force {
            client.clear()?;
        }
        client.download()?;

        Ok(())
    }
}

impl SubmitCommand {
    pub fn run(&self, opt: &RootOpt) -> Result<(), anyhow::Error> {
        log::info!("Running submit command");
        let client = Client::new(opt)?;
        let res = client.client.submit_answer(opt.part as i64, self.answer)?;
        println!("{:?}", res);
        Ok(())
    }
}
