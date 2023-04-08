use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub index: i32,
    #[serde(alias = "pageSize")]
    pub page_size: i32,
    #[serde(alias = "resultCount")]
    pub result_count: i32,
    #[serde(alias = "totalCount")]
    pub total_count: i32,
}