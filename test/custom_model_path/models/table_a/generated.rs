/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Struct representing a row in table `tableA`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName)]
#[diesel(table_name=tableA, primary_key(_id))]
pub struct TableA {
    /// Field representing column `_id`
    pub _id: i32,
}

/// Create Struct for a row in table `tableA` for [`TableA`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=tableA)]
pub struct CreateTableA {
    /// Field representing column `_id`
    pub _id: i32,
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

impl TableA {
    /// Insert a new row into `tableA` with a given [`CreateTableA`]
    pub fn create(db: &mut ConnectionType, item: &CreateTableA) -> diesel::QueryResult<Self> {
        use crate::schema::tableA::dsl::*;

        diesel::insert_into(tableA).values(item).get_result::<Self>(db)
    }

    /// Get a row from `tableA`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param__id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::tableA::dsl::*;

        tableA.filter(_id.eq(param__id)).first::<Self>(db)
    }

    /// Delete a row in `tableA`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param__id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::tableA::dsl::*;

        diesel::delete(tableA.filter(_id.eq(param__id))).execute(db)
    }
}
