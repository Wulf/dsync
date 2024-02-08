/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Struct representing a row in table `todos2`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName)]
#[diesel(table_name=todos2, primary_key(id))]
pub struct Todos2 {
    /// Field representing column `id`
    pub id: i32,
}

/// Result of a `.paginate` function
#[derive(Debug, serde::Serialize)]
pub struct PaginationResult<T> {
    /// Resulting items that are from the current page
    pub items: Vec<T>,
    /// The count of total items there are
    pub total_items: i64,
    /// Current page, 0-based index
    pub page: i64,
    /// Size of a page
    pub page_size: i64,
    /// Number of total possible pages, given the `page_size` and `total_items`
    pub num_pages: i64,
}

impl Todos2 {
    /// Insert a new row into `todos2` with all default values
    pub fn create(db: &mut ConnectionType) -> diesel::QueryResult<Self> {
        use crate::schema::todos2::dsl::*;
        diesel::insert_into(todos2).default_values().get_result::<Self>(db)
    }

    /// Get a row from `todos2`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::todos2::dsl::*;

        todos2.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Delete a row in `todos2`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::todos2::dsl::*;

        diesel::delete(todos2.filter(id.eq(param_id))).execute(db)
    }
}
