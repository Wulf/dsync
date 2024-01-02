/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use serde::{Deserialize, Serialize};
use diesel::QueryResult;

type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row in table `user`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name=user, primary_key(id))]
pub struct User {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `phone_numbers`
    pub phone_numbers: Option<Vec<Option<String>>>,
    /// Field representing column `optional_array_column`
    pub optional_array_column: Option<Vec<Option<String>>>,
}

/// Create Struct for a row in table `user` for [`User`]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name=user)]
pub struct CreateUser {
    /// Field representing column `phone_numbers`
    pub phone_numbers: Option<Vec<Option<String>>>,
    /// Field representing column `optional_array_column`
    pub optional_array_column: Option<Vec<Option<String>>>,
}

/// Update Struct for a row in table `user` for [`User`]
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, Default)]
#[diesel(table_name=user)]
pub struct UpdateUser {
    /// Field representing column `phone_numbers`
    pub phone_numbers: Option<Option<Vec<Option<String>>>>,
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
    pub fn create(db: &mut ConnectionType, item: &CreateUser) -> QueryResult<Self> {
        use crate::schema::user::dsl::*;

        insert_into(user).values(item).get_result::<Self>(db)
    }

    /// Get a row from `user`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::schema::user::dsl::*;

        user.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::user::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = user.count().get_result(db)?;
        let items = user.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row in `user`, identified by the primary key with [`UpdateUser`]
    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateUser) -> QueryResult<Self> {
        use crate::schema::user::dsl::*;

        diesel::update(user.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `user`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::schema::user::dsl::*;

        diesel::delete(user.filter(id.eq(param_id))).execute(db)
    }
}
