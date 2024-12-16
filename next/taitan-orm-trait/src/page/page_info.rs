#[derive(Clone, Debug)]
pub struct PageInfo {
    pub page_size: u64,
    pub page_num: u64,
    pub page_total: u64,
    pub total: u64,
}

impl PageInfo {
    pub fn empty(page_size: u64, page_num: u64) -> Self {
        Self {
            page_size,
            page_num,
            page_total: 0,
            total: 0,
        }
    }
}
