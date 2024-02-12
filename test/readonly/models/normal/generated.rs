/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Struct representing a row in table `normal`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName)]
#[diesel(table_name=normal, primary_key(id))]
pub struct Normal {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `testprop`
    pub testprop: i32,
}

/// Create Struct for a row in table `normal` for [`Normal`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=normal)]
pub struct CreateNormal {
    /// Field representing column `testprop`
    pub testprop: i32,
}

/// Update Struct for a row in table `normal` for [`Normal`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=normal)]
pub struct UpdateNormal {
    /// Field representing column `testprop`
    pub testprop: Option<i32>,
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

impl Normal {
    /// Insert a new row into `normal` with a given [`CreateNormal`]
    pub fn create(db: &mut ConnectionType, item: &CreateNormal) -> diesel::QueryResult<Self> {
        use crate::schema::normal::dsl::*;

        diesel::insert_into(normal).values(item).get_result::<Self>(db)
    }

    /// Get a row from `normal`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::normal::dsl::*;

        normal.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `normal`, identified by the primary key with [`UpdateNormal`]
    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateNormal) -> diesel::QueryResult<Self> {
        use crate::schema::normal::dsl::*;

        diesel::update(normal.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `normal`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::normal::dsl::*;

        diesel::delete(normal.filter(id.eq(param_id))).execute(db)
    }
}
