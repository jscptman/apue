use std::fs::FileType;
use std::os::windows::fs::FileTypeExt as StdFileTypeExt;

pub trait FileTypeExt {
    fn get_current_type(&self) -> FileTypeEnum;
}

impl FileTypeExt for FileType {
    fn get_current_type(&self) -> FileTypeEnum {
        if self.is_file() {
            FileTypeEnum::Regular
        } else if self.is_dir() {
            FileTypeEnum::Directory
        } else if self.is_symlink_file() {
            FileTypeEnum::SymbolLink(SymbolLinkType::Regular)
        } else if self.is_symlink_dir() {
            FileTypeEnum::SymbolLink(SymbolLinkType::Directory)
        } else {
            FileTypeEnum::Unknown
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum FileTypeEnum {
    Regular,
    SymbolLink(SymbolLinkType),
    Directory,
    Unknown,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum SymbolLinkType {
    Regular,
    Directory,
}
