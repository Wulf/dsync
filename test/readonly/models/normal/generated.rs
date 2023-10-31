/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use serde::{Deserialize, Serialize};
use diesel::QueryResult;

type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row in table `normal`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=normal, primary_key(id))]
pub struct Normal {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `testprop`
    pub testprop: i32,
}

/// Create Struct for a row in table `normal` for [`Normal`]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name=normal)]
pub struct CreateNormal {
    /// Field representing column `testprop`
    pub testprop: i32,
}

/// Update Struct for a row in table `normal` for [`Normal`]
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, Default)]
#[diesel(table_name=normal)]
pub struct UpdateNormal {
    /// Field representing column `testprop`
    pub testprop: Option<i32>,
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

impl Normal {
    /// Insert a new row into `normal` with a given [`CreateNormal`]
    pub fn create(db: &mut ConnectionType, item: &CreateNormal) -> QueryResult<Self> {
        use crate::schema::normal::dsl::*;

        insert_into(normal).values(item).get_result::<Self>(db)
    }

    /// Get a row from `normal`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::schema::normal::dsl::*;

        normal.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::normal::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = normal.count().get_result(db)?;
        let items = normal.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row in `normal`, identified by the primary key with [`UpdateNormal`]
    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateNormal) -> QueryResult<Self> {
        use crate::schema::normal::dsl::*;

        diesel::update(normal.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `normal`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::schema::normal::dsl::*;

        diesel::delete(normal.filter(id.eq(param_id))).execute(db)
    }
}
