/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Struct representing a row in table `fang_tasks`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName)]
#[diesel(table_name=fang_tasks, primary_key(id))]
pub struct FangTasks {
    /// Field representing column `id`
    pub id: uuid::Uuid,
    /// Field representing column `metadata`
    pub metadata: serde_json::Value,
    /// Field representing column `error_message`
    pub error_message: Option<String>,
    /// Field representing column `state`
    pub state: crate::schema::sql_types::FangTaskState,
    /// Field representing column `task_type`
    pub task_type: String,
    /// Field representing column `uniq_hash`
    pub uniq_hash: Option<String>,
    /// Field representing column `retries`
    pub retries: i32,
    /// Field representing column `scheduled_at`
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    /// Field representing column `created_at`
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Field representing column `updated_at`
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Create Struct for a row in table `fang_tasks` for [`FangTasks`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=fang_tasks)]
pub struct CreateFangTasks {
    /// Field representing column `id`
    pub id: uuid::Uuid,
    /// Field representing column `metadata`
    pub metadata: serde_json::Value,
    /// Field representing column `error_message`
    pub error_message: Option<String>,
    /// Field representing column `state`
    pub state: crate::schema::sql_types::FangTaskState,
    /// Field representing column `task_type`
    pub task_type: String,
    /// Field representing column `uniq_hash`
    pub uniq_hash: Option<String>,
    /// Field representing column `retries`
    pub retries: i32,
    /// Field representing column `scheduled_at`
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    /// Field representing column `created_at`
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Field representing column `updated_at`
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Update Struct for a row in table `fang_tasks` for [`FangTasks`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=fang_tasks)]
pub struct UpdateFangTasks {
    /// Field representing column `metadata`
    pub metadata: Option<serde_json::Value>,
    /// Field representing column `error_message`
    pub error_message: Option<Option<String>>,
    /// Field representing column `state`
    pub state: Option<crate::schema::sql_types::FangTaskState>,
    /// Field representing column `task_type`
    pub task_type: Option<String>,
    /// Field representing column `uniq_hash`
    pub uniq_hash: Option<Option<String>>,
    /// Field representing column `retries`
    pub retries: Option<i32>,
    /// Field representing column `scheduled_at`
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
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

impl FangTasks {
    /// Insert a new row into `fang_tasks` with a given [`CreateFangTasks`]
    pub fn create(db: &mut ConnectionType, item: &CreateFangTasks) -> diesel::QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        diesel::insert_into(fang_tasks).values(item).get_result::<Self>(db)
    }

    /// Get a row from `fang_tasks`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: uuid::Uuid) -> diesel::QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        fang_tasks.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `fang_tasks`, identified by the primary key with [`UpdateFangTasks`]
    pub fn update(db: &mut ConnectionType, param_id: uuid::Uuid, item: &UpdateFangTasks) -> diesel::QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        diesel::update(fang_tasks.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `fang_tasks`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: uuid::Uuid) -> diesel::QueryResult<usize> {
        use crate::schema::fang_tasks::dsl::*;

        diesel::delete(fang_tasks.filter(id.eq(param_id))).execute(db)
    }
}
