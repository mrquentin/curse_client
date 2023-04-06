use serde::Deserialize;

#[derive(Deserialize)]
pub struct Category {
    pub id: i32,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub name: String,
    pub slug: String,
    pub url: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "isClass")]
    pub is_class: bool,
    #[serde(rename = "classId")]
    pub class_id: i32,
    #[serde(rename = "parentCategoryId")]
    pub parent_category_id: i32,
    #[serde(rename = "displayIndex")]
    pub display_index: i32,
}

#[derive(Deserialize)]
pub struct GetCategoriesResponse {
    pub data: Vec<Category>,
}