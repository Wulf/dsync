/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row in table `tableSuffix`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name=tableSuffix, primary_key(id))]
pub struct TableSuffix {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `testprop`
    pub testprop: i32,
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

impl TableSuffix {
    /// Get a row from `tableSuffix`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::schema::tableSuffix::dsl::*;

        tableSuffix.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::tableSuffix::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = tableSuffix.count().get_result(db)?;
        let items = tableSuffix.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }
}
