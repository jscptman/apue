use std::fs::FileType;
use std::os::unix::fs::FileTypeExt;

pub trait RFileTypeExt {
    fn get_current_type(&self) -> FileTypeEnum;
}

impl RFileTypeExt for FileType {
    fn get_current_type(&self) -> FileTypeEnum {
        if self.is_file() {
            FileTypeEnum::Regular
        } else if self.is_dir() {
            FileTypeEnum::Directory
        } else if self.is_block_device() {
            FileTypeEnum::BlockDevice
        } else if self.is_char_device() {
            FileTypeEnum::CharDevice
        } else if self.is_fifo() {
            FileTypeEnum::Fifo
        } else if self.is_socket() {
            FileTypeEnum::Socket
        } else if self.is_symlink() {
            FileTypeEnum::SymbolLink
        } else {
            FileTypeEnum::Unknown
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum FileTypeEnum {
    Regular,
    SymbolLink,
    Directory,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
    Unknown,
}
