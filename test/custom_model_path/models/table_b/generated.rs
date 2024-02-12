/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::data::models::table_a::TableA;
use crate::schema::*;

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
    pub fn create(db: &mut ConnectionType, item: &CreateTableB) -> diesel::QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        diesel::insert_into(tableB).values(item).get_result::<Self>(db)
    }

    /// Get a row from `tableB`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param__id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        tableB.filter(_id.eq(param__id)).first::<Self>(db)
    }

    /// Update a row in `tableB`, identified by the primary key with [`UpdateTableB`]
    pub fn update(db: &mut ConnectionType, param__id: i32, item: &UpdateTableB) -> diesel::QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        diesel::update(tableB.filter(_id.eq(param__id))).set(item).get_result(db)
    }

    /// Delete a row in `tableB`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param__id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::tableB::dsl::*;

        diesel::delete(tableB.filter(_id.eq(param__id))).execute(db)
    }
}
