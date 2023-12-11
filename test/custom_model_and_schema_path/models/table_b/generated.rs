/* @generated and managed by dsync */

use crate::diesel::*;
use crate::data::models::table_a::TableA;
use crate::data::schema::*;
use diesel::QueryResult;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Struct representing a row in table `tableB`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Associations, diesel::Identifiable)]
#[diesel(table_name=tableB, primary_key(_id), belongs_to(TableA, foreign_key=link))]
pub struct TableB {
    /// Field representing column `_id`
    pub _id: i32,
    /// Field representing column `link`
    pub link: i32,
}

/// Create Struct for a row in table `tableB` for [`TableB`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=tableB)]
pub struct CreateTableB {
    /// Field representing column `_id`
    pub _id: i32,
    /// Field representing column `link`
    pub link: i32,
}

/// Update Struct for a row in table `tableB` for [`TableB`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=tableB)]
pub struct UpdateTableB {
    /// Field representing column `link`
    pub link: Option<i32>,
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

impl TableB {
    /// Insert a new row into `tableB` with a given [`CreateTableB`]
    pub fn create(db: &mut ConnectionType, item: &CreateTableB) -> QueryResult<Self> {
        use crate::data::schema::tableB::dsl::*;

        insert_into(tableB).values(item).get_result::<Self>(db)
    }

    /// Get a row from `tableB`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param__id: i32) -> QueryResult<Self> {
        use crate::data::schema::tableB::dsl::*;

        tableB.filter(_id.eq(param__id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::data::schema::tableB::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = tableB.count().get_result(db)?;
        let items = tableB.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row in `tableB`, identified by the primary key with [`UpdateTableB`]
    pub fn update(db: &mut ConnectionType, param__id: i32, item: &UpdateTableB) -> QueryResult<Self> {
        use crate::data::schema::tableB::dsl::*;

        diesel::update(tableB.filter(_id.eq(param__id))).set(item).get_result(db)
    }

    /// Delete a row in `tableB`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param__id: i32) -> QueryResult<usize> {
        use crate::data::schema::tableB::dsl::*;

        diesel::delete(tableB.filter(_id.eq(param__id))).execute(db)
    }
}
