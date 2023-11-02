/* @generated and managed by dsync */
#[derive(Debug, )]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    /// 0-based index
    pub page: i64,
    pub page_size: i64,
    pub num_pages: i64,
}

pub type ConnectionType = diesel::pg::PgConnection;
