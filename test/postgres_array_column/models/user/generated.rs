/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Struct representing a row in table `user`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName)]
#[diesel(table_name=user, primary_key(id))]
pub struct User {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `phone_numbers`
    pub phone_numbers: Vec<Option<String>>,
    /// Field representing column `optional_array_column`
    pub optional_array_column: Option<Vec<Option<String>>>,
}

/// Create Struct for a row in table `user` for [`User`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=user)]
pub struct CreateUser {
    /// Field representing column `phone_numbers`
    pub phone_numbers: Vec<Option<String>>,
    /// Field representing column `optional_array_column`
    pub optional_array_column: Option<Vec<Option<String>>>,
}

/// Update Struct for a row in table `user` for [`User`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=user)]
pub struct UpdateUser {
    /// Field representing column `phone_numbers`
    pub phone_numbers: Option<Vec<Option<String>>>,
    /// Field representing column `optional_array_column`
    pub optional_array_column: Option<Option<Vec<Option<String>>>>,
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

impl User {
    /// Insert a new row into `user` with a given [`CreateUser`]
    pub fn create(db: &mut ConnectionType, item: &CreateUser) -> diesel::QueryResult<Self> {
        use crate::schema::user::dsl::*;

        diesel::insert_into(user).values(item).get_result::<Self>(db)
    }

    /// Get a row from `user`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::user::dsl::*;

        user.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `user`, identified by the primary key with [`UpdateUser`]
    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateUser) -> diesel::QueryResult<Self> {
        use crate::schema::user::dsl::*;

        diesel::update(user.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `user`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::user::dsl::*;

        diesel::delete(user.filter(id.eq(param_id))).execute(db)
    }
}
