/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
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

    /// Paginates through the table where page is a 1-based index (i.e. page 1 is the first page)
    pub fn paginate(db: &mut ConnectionType, param_page_starting_with_1: i64, param_page_size: i64, filter: FangTasksFilter) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::fang_tasks::dsl::*;

        let param_page = param_page_starting_with_1.max(0);
        let param_page_size = param_page_size.max(1);
        let total_items = Self::filter(filter.clone()).count().get_result(db)?;
        let items = Self::filter(filter).limit(param_page_size).offset(param_page * param_page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page: param_page,
            page_size: param_page_size,
            /* ceiling division of integers */
            num_pages: total_items / param_page_size + i64::from(total_items % param_page_size != 0)
        })
    }

    /// A utility function to help build custom search queries
    /// 
    /// Example:
    /// 
    pub fn filter<'a>(
        filter: FangTasksFilter,
    ) -> crate::schema::fang_tasks::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::fang_tasks::table.into_boxed();
        
        
        if let Some(filter_id) = filter.id.clone() {
            query = query.filter(crate::schema::fang_tasks::id.eq(filter_id));
        }
        if let Some(filter_metadata) = filter.metadata.clone() {
            query = query.filter(crate::schema::fang_tasks::metadata.eq(filter_metadata));
        }
        if let Some(filter_error_message) = filter.error_message.clone() {
            query = query.filter(crate::schema::fang_tasks::error_message.eq(filter_error_message));
        }
        if let Some(filter_state) = filter.state.clone() {
            query = query.filter(crate::schema::fang_tasks::state.eq(filter_state));
        }
        if let Some(filter_task_type) = filter.task_type.clone() {
            query = query.filter(crate::schema::fang_tasks::task_type.eq(filter_task_type));
        }
        if let Some(filter_uniq_hash) = filter.uniq_hash.clone() {
            query = query.filter(crate::schema::fang_tasks::uniq_hash.eq(filter_uniq_hash));
        }
        if let Some(filter_retries) = filter.retries.clone() {
            query = query.filter(crate::schema::fang_tasks::retries.eq(filter_retries));
        }
        if let Some(filter_scheduled_at) = filter.scheduled_at.clone() {
            query = query.filter(crate::schema::fang_tasks::scheduled_at.eq(filter_scheduled_at));
        }
        if let Some(filter_created_at) = filter.created_at.clone() {
            query = query.filter(crate::schema::fang_tasks::created_at.eq(filter_created_at));
        }
        if let Some(filter_updated_at) = filter.updated_at.clone() {
            query = query.filter(crate::schema::fang_tasks::updated_at.eq(filter_updated_at));
        }
        
        query
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
#[derive(Clone)]
pub struct FangTasksFilter {
    pub id: Option<uuid::Uuid>,
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

impl Default for FangTasksFilter {
    fn default() -> FangTasksFilter {
        FangTasksFilter {
            id: None,
            metadata: None,
            error_message: None,
            state: None,
            task_type: None,
            uniq_hash: None,
            retries: None,
            scheduled_at: None,
            created_at: None,
            updated_at: None,
        }
    }
}
