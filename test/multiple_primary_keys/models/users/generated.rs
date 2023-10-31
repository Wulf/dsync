/* @generated and managed by dsync */

use diesel::*;
use crate::schema::*;
use serde::{Deserialize, Serialize};
use diesel::QueryResult;

type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row in table `users`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=users, primary_key(name,address))]
pub struct Users {
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `secret`
    pub secret: String,
}

/// Create Struct for a row in table `users` for [`Users`]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name=users)]
pub struct CreateUsers {
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `secret`
    pub secret: String,
}

/// Update Struct for a row in table `users` for [`Users`]
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, Default)]
#[diesel(table_name=users)]
pub struct UpdateUsers {
    /// Field representing column `secret`
    pub secret: Option<String>,
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

impl Users {
    /// Insert a new row into `users` with a given [`CreateUsers`]
    pub fn create(db: &mut ConnectionType, item: &CreateUsers) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        insert_into(users).values(item).get_result::<Self>(db)
    }

    /// Get a row from `users`, identified by the primary keys
    pub fn read(db: &mut ConnectionType, param_name: String, param_address: String) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        users.filter(name.eq(param_name)).filter(address.eq(param_address)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::users::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = users.count().get_result(db)?;
        let items = users.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row in `users`, identified by the primary keys with [`UpdateUsers`]
    pub fn update(db: &mut ConnectionType, param_name: String, param_address: String, item: &UpdateUsers) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        diesel::update(users.filter(name.eq(param_name)).filter(address.eq(param_address))).set(item).get_result(db)
    }

    /// Delete a row in `users`, identified by the primary keys
    pub fn delete(db: &mut ConnectionType, param_name: String, param_address: String) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(name.eq(param_name)).filter(address.eq(param_address))).execute(db)
    }
}
