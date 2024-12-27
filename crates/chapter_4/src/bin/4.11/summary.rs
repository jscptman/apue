#[cfg(target_family = "windows")]
use apue::os::windows::fs::file_type_etx::{FileTypeEnum, FileTypeExt};
#[cfg(target_family = "unix")]
use apue_common::os::unix::fs::file_type_etx::{FileTypeEnum, FileTypeExt};
use std::{collections::HashMap, fs::Metadata};

type Summary = HashMap<FileTypeEnum, Vec<Metadata>>;
pub struct FileStatisticSummary {
    file_summary_map: Summary,
    total: usize,
}

impl FileStatisticSummary {
    fn compute_total(&mut self) -> usize {
        self.file_summary_map
            .values()
            .fold(0, |acc, cur| acc + cur.len())
    }
    pub fn push(&mut self, data: Metadata) {
        let file_type = data.file_type().get_current_type();
        if let Some(summary) = self.file_summary_map.get_mut(&file_type) {
            summary.push(data);
        } else {
            self.file_summary_map.insert(file_type, Vec::from([data]));
        }
        self.total = self.compute_total();
    }
    pub fn new() -> Self {
        FileStatisticSummary {
            total: 0,
            file_summary_map: HashMap::new(),
        }
    }
    pub fn total(&self) -> usize {
        self.total
    }
    pub fn summary(&self) -> &Summary {
        &self.file_summary_map
    }
}
