use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub index: Option<i32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    #[serde(rename = "resultCount")]
    pub result_count: Option<i32>,
    #[serde(rename = "totalCount")]
    pub total_count: Option<i32>,
}