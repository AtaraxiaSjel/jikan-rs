use serde::{
    Deserialize,
    Serialize,
};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PaginationItems {
    pub count: u64,
    pub total: u64,
    pub per_page: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Pagination {
    pub last_visible_page: u64,
    pub has_next_page: bool,
    pub items: Option<PaginationItems>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Root<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}
