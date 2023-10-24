use serde::{Deserialize};
use super::From;

pub struct Yml;

impl From for Yml {
    fn from_content<T>(&self, content: &str) -> T
        where T: for<'de> Deserialize<'de>
    {
        let target = serde_yaml::from_str(content).expect(".yml file parse failed.");
        target
    }
}
