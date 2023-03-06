use std::{
    fs::{self, Metadata},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::{DirEntry, WalkDir};

type FilterFn = fn(&DirEntry) -> bool;

pub trait CustomPath {
    fn get_most_recent_modified_date(&self, max_depth: usize, filters: &[FilterFn]) -> SystemTime;
}

impl CustomPath for Path {
    fn get_most_recent_modified_date(&self, max_depth: usize, filters: &[FilterFn]) -> SystemTime {
        let mut most_recent: Option<SystemTime> = None;

        for entry in WalkDir::new(self)
            .max_depth(max_depth)
            .into_iter()
            .filter_entry(|d| filters.iter().all(|f| !f(d)))
            .filter_map(|e| e.ok())
        {
            let metadata: Metadata = match fs::metadata(entry.path()) {
                Ok(metadata) => metadata,
                Err(_) => continue,
            };

            match metadata.modified() {
                Ok(modified) => {
                    if most_recent.is_none() || modified > most_recent.unwrap() {
                        most_recent = Some(modified);
                    }
                }
                Err(_) => continue,
            }
        }

        most_recent.unwrap_or(UNIX_EPOCH)
    }
}
