use crate::{XError, XResult};
use calamine::{Data, Reader};
use pathdiff::diff_paths;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};
use twox_hash::XxHash64;

use xcell_core::Itertools;

pub use self::workspace::*;

mod watcher;

pub mod comment;
pub mod file_format;
mod workspace;



/// 确保第一行的 id 不是空的
///
/// 如果是空的, 那么就认为数据非法
pub fn first_not_nil(row: &[Data]) -> bool {
    match row.first() {
        Some(s) => match s {
            Data::Int(_) => true,
            Data::Float(_) => true,
            Data::String(s) => !s.is_empty(),
            Data::Bool(_) => true,
            Data::DateTime(_) => true,
            Data::DateTimeIso(_) => true,
            Data::DurationIso(_) => true,
            Data::Error(_) => false,
            Data::Empty => false,
        },
        None => false,
    }
}

pub fn xx_hash<T>(body: T) -> u64
where
    T: Hash,
{
    let mut hasher = XxHash64::default();
    body.hash(&mut hasher);
    hasher.finish()
}

pub fn xx_file(path: &Path) -> XResult<u64> {
    let mut hasher = XxHash64::default();
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.write(&buffer[..count]);
    }
    Ok(hasher.finish())
}

pub fn split_file_name(s: &str) -> String {
    let mut all = vec![];
    for name in s.split(|c| c == '/' || c == '\\') {
        if !name.trim().is_empty() {
            all.push(name)
        }
    }
    all.join("/")
}

pub fn split_namespace(s: &str) -> Vec<&str> {
    let mut all = vec![];
    for s in s.split("::") {
        for name in s.split('.') {
            if !name.trim().is_empty() {
                all.push(name)
            }
        }
    }
    all
}

pub use xcell_parser::norm_string;
