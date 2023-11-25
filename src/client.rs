use crate::RootOpt;
use aoc_client::AocClient;
use clap::Parser;
use std::path::PathBuf;

pub struct Client {
    pub client: AocClient,
    pub assignment_path: PathBuf,
    pub input_path: PathBuf,
}

#[derive(Parser, Debug, Clone)]
pub struct DownloadCommand {}

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
        })
    }
}

impl DownloadCommand {
    pub fn run(&self, opt: &RootOpt) -> Result<(), anyhow::Error> {
        log::info!("Running download command");
        log::info!("Day: {}", opt.day);
        log::info!("Part: {}", opt.part);

        let client = Client::new(opt)?;

        if !client.assignment_path.exists() {
            client.client.save_puzzle_markdown()?;
        }

        if !client.input_path.exists() {
            let input = client.client.get_input()?;
            std::fs::write(format!("./input/day{:02}.txt", opt.day), input)?;
        }

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
