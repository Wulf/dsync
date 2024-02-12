/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Struct representing a row in table `todos`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todos {
    /// Field representing column `id`
    pub id: i32,
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=todos)]
pub struct CreateTodos {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `text`
    pub text: String,
    /// Field representing column `completed`
    pub completed: bool,
}

/// Update Struct for a row in table `todos` for [`Todos`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodos {
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
    pub fn create(db: &mut ConnectionType, item: &CreateTodos) -> diesel::QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        diesel::insert_into(todos).values(item).get_result::<Self>(db)
    }

    /// Get a row from `todos`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        todos.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `todos`, identified by the primary key with [`UpdateTodos`]
    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateTodos) -> diesel::QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        diesel::update(todos.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `todos`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(param_id))).execute(db)
    }
}
