/* @generated and managed by dsync */

use crate::diesel::*;
use crate::data::schema::*;
use diesel::QueryResult;

type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row in table `todos`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todos {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `unsigned`
    pub unsigned: u32,
    /// Field representing column `text`
    pub text: String,
    /// Field representing column `completed`
    pub completed: bool,
    /// Field representing column `created_at`
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Field representing column `updated_at`
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Create Struct for a row in table `todos` for [`Todos`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Insertable)]
#[diesel(table_name=todos)]
pub struct CreateTodos {
    /// Field representing column `unsigned`
    pub unsigned: u32,
    /// Field representing column `text`
    pub text: String,
    /// Field representing column `completed`
    pub completed: bool,
}

/// Update Struct for a row in table `todos` for [`Todos`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, AsChangeset, PartialEq, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodos {
    /// Field representing column `unsigned`
    pub unsigned: Option<u32>,
    /// Field representing column `text`
    pub text: Option<String>,
    /// Field representing column `completed`
    pub completed: Option<bool>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Field representing column `updated_at`
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
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

impl Todos {
    /// Insert a new row into `todos` with a given [`CreateTodos`]
    pub fn create(db: &mut ConnectionType, item: &CreateTodos) -> QueryResult<Self> {
        use crate::data::schema::todos::dsl::*;

        insert_into(todos).values(item).get_result::<Self>(db)
    }

    /// Get a row from `todos`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::data::schema::todos::dsl::*;

        todos.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::data::schema::todos::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = todos.count().get_result(db)?;
        let items = todos.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row in `todos`, identified by the primary key with [`UpdateTodos`]
    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateTodos) -> QueryResult<Self> {
        use crate::data::schema::todos::dsl::*;

        diesel::update(todos.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `todos`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::data::schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(param_id))).execute(db)
    }
}
