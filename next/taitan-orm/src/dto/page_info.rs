#[derive(Clone, Debug)]
pub struct PageInfo {
    pub page_size: usize,
    pub page_num: usize,
    pub page_total: usize,
    pub total: usize,
}

impl PageInfo {
    pub fn empty(page_size: usize, page_num: usize) -> Self {
        Self {
            page_size,
            page_num,
            page_total: 0,
            total: 0,
        }
    }
}