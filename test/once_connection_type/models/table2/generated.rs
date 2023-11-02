/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use crate::models::common::*;

/// Struct representing a row in table `table2`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name=table2, primary_key(id))]
pub struct Table2 {
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

impl Table2 {
    /// Insert a new row into `table2` with all default values
    pub fn create(db: &mut ConnectionType) -> QueryResult<Self> {
        use crate::schema::table2::dsl::*;

        insert_into(table2).default_values().get_result::<Self>(db)
    }

    /// Get a row from `table2`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::schema::table2::dsl::*;

        table2.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::table2::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = table2.count().get_result(db)?;
        let items = table2.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Delete a row in `table2`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::schema::table2::dsl::*;

        diesel::delete(table2.filter(id.eq(param_id))).execute(db)
    }
}
