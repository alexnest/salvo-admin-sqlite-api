use std::fs::{self, File};
use std::io::Read;
use serde::{Deserialize};

pub mod toml;
pub mod yml;

pub trait From {
    fn from_path<T>(&self, path: &str) -> T
        where T: for<'de> Deserialize<'de>
    {
        let s = fs::read_to_string(path).expect("read file failed");
        let target = self.from_content::<T>(s.as_str());
        target
    }

    fn from_content<T>(&self, content: &str) -> T where T: for<'de> Deserialize<'de>;

    fn from_file<T>(&self, file: &mut File) -> T
        where T: for<'de> Deserialize<'de>
    {
        let mut content = String::new();
        file.read_to_string(&mut content).expect("read file failed");
        let target = self.from_content::<T>(content.as_str());
        target
    }
}