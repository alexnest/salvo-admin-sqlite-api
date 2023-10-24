use crate::global::cst::SEPARATOR;

pub struct Level<'a> {
    parent_level: &'a str,
    parent_id: &'a str,
}

impl<'a> Level<'a> {
    pub fn new(parent_level: &'a str, parent_id: &'a str) -> Self {
        Self {
            parent_level,
            parent_id,
        }
    }

    pub fn cal(&self) -> String {
        format!(
            "{}{}{}",
            self.parent_level.to_string(),
            SEPARATOR,
            self.parent_id.to_string()
        )
    }
}
