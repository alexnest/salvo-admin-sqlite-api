use serde::{Deserialize, Serialize};

// the parameter of page
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub page_no: u32,   // current page number
    pub page_size: u32, // how many items per page
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<'a, T> {
    pub page_no: u32,   // current page number
    pub page_size: u32, // how many items per page
    pub total_count: u32, // total items count
    pub total_page: u32,  // total pages count
    pub items: &'a Vec<T>,
}

impl<'a, T> PageResult<'a, T> {
    pub fn new(page: &Page, total_count: u32, items: &'a Vec<T>) -> Self {
        let total_page = if total_count % page.page_size == 0 {
            total_count / page.page_size
        } else {
            total_count / page.page_size + 1
        };
        Self {
            page_no: page.page_no,
            page_size: page.page_size,
            total_count,
            total_page,
            items,
        }
    }
}

impl Page {
    pub fn offset(&self) -> i64 {
        ((self.page_no - 1) * self.page_size) as i64
    }
}
