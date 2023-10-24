use serde::{Deserialize};
use super::From;

pub struct Toml;

impl From for Toml {
    fn from_content<T>(&self, content: &str) -> T
        where T: for<'de> Deserialize<'de>
    {
        let target = toml::from_str(content).expect(".toml file parse failed.");
        target
    }
}