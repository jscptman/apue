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
impl FileTypeEnum {
    pub fn convert_str(&self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::SymbolLink => "symlink",
            Self::Directory => "directory",
            Self::CharDevice => "char_device",
            Self::BlockDevice => "block_device",
            Self::Fifo => "fifo",
            Self::Socket => "socket",
            Self::Unknown => "unknown",
        }
    }
}
pub const FILE_TYPES_COUNT: usize = 8;
pub const FILE_TYPES: [FileTypeEnum; FILE_TYPES_COUNT] = [
    FileTypeEnum::Regular,
    FileTypeEnum::SymbolLink,
    FileTypeEnum::Directory,
    FileTypeEnum::BlockDevice,
    FileTypeEnum::CharDevice,
    FileTypeEnum::Fifo,
    FileTypeEnum::Socket,
    FileTypeEnum::Unknown,
];
