use r::common_mod::r_error::MsgError;
use std::{collections::HashMap, fs::Metadata};
#[cfg(target_family = "unix")]
use r::common_mod::r_os::unix::r_fs::r_file_type_etx::{FileTypeEnum, RFileTypeExt};
#[cfg(target_family = "windows")]
use r::common_mod::r_os::windows::r_fs::r_file_type_etx::{FileTypeEnum, RFileTypeExt};

pub struct SingleTypeSummaryMetaData {
    meta_data: Vec<Metadata>,
    file_type: FileTypeEnum,
}

impl SingleTypeSummaryMetaData {
    pub fn total(&self) -> usize {
        self.meta_data.len()
    }
    pub fn from(meta_data: Vec<Metadata>) -> Result<Self, MsgError<&'static str>> {
        if meta_data.len() == 0 {
            panic!("method occurs an error: SingleTypeSummary::from arguments.len() should not equal 0")
        }
        let file_type = meta_data[0].file_type().get_current_type();
        Ok(Self {
            meta_data,
            file_type,
        })
    }
    pub fn file_type(&self) -> &FileTypeEnum {
        &self.file_type
    }
}
pub struct FileStatisticSummary {
    file_summary_map: HashMap<FileTypeEnum, SingleTypeSummaryMetaData>,
    total: usize,
}

impl FileStatisticSummary {
    pub fn compute_total(&mut self) -> usize {
        self.file_summary_map
            .values()
            .fold(0, |acc, cur| acc + cur.total())
    }
    pub fn insert(&mut self, mut data: SingleTypeSummaryMetaData) {
        let file_type = data.file_type();
        let insert_count = data.total();
        if let Some(summary) = self.file_summary_map.get_mut(file_type) {
            summary.meta_data.append(&mut data.meta_data);
        } else {
            self.file_summary_map.insert(file_type.clone(), data);
        }
        self.total += insert_count;
    }
    pub fn new() -> Self {
        FileStatisticSummary {
            total: 0,
            file_summary_map: HashMap::new(),
        }
    }
}

