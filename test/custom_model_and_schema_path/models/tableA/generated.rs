/* @generated and managed by dsync */

use crate::diesel::*;
use crate::data::schema::*;
use serde::{Deserialize, Serialize};
use diesel::QueryResult;

type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row in table `tableA`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name=tableA, primary_key(_id))]
pub struct TableA {
    /// Field representing column `_id`
    pub _id: i32,
}

/// Create Struct for a row in table `tableA` for [`TableA`]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name=tableA)]
pub struct CreateTableA {
    /// Field representing column `_id`
    pub _id: i32,
}

/// Result of a `.paginate` function
#[derive(Debug, Serialize)]
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

impl TableA {
    /// Insert a new row into `tableA` with a given [`CreateTableA`]
    pub fn create(db: &mut ConnectionType, item: &CreateTableA) -> QueryResult<Self> {
        use crate::data::schema::tableA::dsl::*;

        insert_into(tableA).values(item).get_result::<Self>(db)
    }

    /// Get a row from `tableA`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param__id: i32) -> QueryResult<Self> {
        use crate::data::schema::tableA::dsl::*;

        tableA.filter(_id.eq(param__id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::data::schema::tableA::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = tableA.count().get_result(db)?;
        let items = tableA.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Delete a row in `tableA`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param__id: i32) -> QueryResult<usize> {
        use crate::data::schema::tableA::dsl::*;

        diesel::delete(tableA.filter(_id.eq(param__id))).execute(db)
    }
}
