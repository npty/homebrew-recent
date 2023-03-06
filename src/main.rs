mod file_info;
mod filters;
mod path;

use chrono::prelude::*;
use clap::Parser;
use dirs;
use file_info::file_info::FileInfo;
use filters::filters::{is_hidden, is_node_modules};
use path::CustomPath;
use std::fs::{self};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 1)]
    pub size: u8,

    #[arg(short, long, default_value_t = false)]
    pub display_date: bool,
}

const MAX_DEPTH: usize = 5;

fn main() {
    let args = Args::parse();
    let path = dirs::home_dir().unwrap().join("workspace");

    let mut data: Vec<FileInfo> = fs::read_dir(path)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();

            FileInfo {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                modified: path
                    .get_most_recent_modified_date(MAX_DEPTH, &[is_hidden, is_node_modules]),
            }
        })
        .collect();

    data.sort_by_key(|file_info| file_info.modified);
    data.reverse();

    data.iter().take(args.size.into()).for_each(|file_info| {
        let mut date = String::new();
        if args.display_date {
            let datetime: DateTime<Local> = file_info.modified.into();
            date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        }
        println!("{:?} {}", file_info.name, date);
    });
}
