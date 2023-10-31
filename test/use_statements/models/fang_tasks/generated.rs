/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use serde::{Deserialize, Serialize};
use diesel::QueryResult;

type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row in table `fang_tasks`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=fang_tasks, primary_key(id))]
pub struct FangTasks {
    pub id: uuid::Uuid,
    pub metadata: serde_json::Value,
    pub error_message: Option<String>,
    pub state: crate::schema::sql_types::FangTaskState,
    pub task_type: String,
    pub uniq_hash: Option<String>,
    pub retries: i32,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Create Struct for a row in table `fang_tasks` for [`FangTasks`]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name=fang_tasks)]
pub struct CreateFangTasks {
    pub id: uuid::Uuid,
    pub metadata: serde_json::Value,
    pub error_message: Option<String>,
    pub state: crate::schema::sql_types::FangTaskState,
    pub task_type: String,
    pub uniq_hash: Option<String>,
    pub retries: i32,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Update Struct for a row in table `fang_tasks` for [`FangTasks`]
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, Default)]
#[diesel(table_name=fang_tasks)]
pub struct UpdateFangTasks {
    pub metadata: Option<serde_json::Value>,
    pub error_message: Option<Option<String>>,
    pub state: Option<crate::schema::sql_types::FangTaskState>,
    pub task_type: Option<String>,
    pub uniq_hash: Option<Option<String>>,
    pub retries: Option<i32>,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    /// 0-based index
    pub page: i64,
    pub page_size: i64,
    pub num_pages: i64,
}

impl FangTasks {
    pub fn create(db: &mut ConnectionType, item: &CreateFangTasks) -> QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        insert_into(fang_tasks).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_id: uuid::Uuid) -> QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        fang_tasks.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::fang_tasks::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = fang_tasks.count().get_result(db)?;
        let items = fang_tasks.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut ConnectionType, param_id: uuid::Uuid, item: &UpdateFangTasks) -> QueryResult<Self> {
        use crate::schema::fang_tasks::dsl::*;

        diesel::update(fang_tasks.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut ConnectionType, param_id: uuid::Uuid) -> QueryResult<usize> {
        use crate::schema::fang_tasks::dsl::*;

        diesel::delete(fang_tasks.filter(id.eq(param_id))).execute(db)
    }
}
