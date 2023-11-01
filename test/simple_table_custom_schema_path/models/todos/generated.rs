/* @generated and managed by dsync */

use crate::diesel::*;
use crate::data::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todos {
    pub id: i32,
    pub unsigned: u32,
    pub text: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct CreateTodos {
    pub unsigned: u32,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodos {
    pub unsigned: Option<u32>,
    pub text: Option<String>,
    pub completed: Option<bool>,
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

impl Todos {

    pub fn create(db: &mut ConnectionType, item: &CreateTodos) -> QueryResult<Self> {
        use crate::data::schema::todos::dsl::*;

        insert_into(todos).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::data::schema::todos::dsl::*;

        todos.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: TodosFilter) -> QueryResult<PaginationResult<Self>> {
        use crate::data::schema::todos::dsl::*;

        let page = page.max(0);
        let page_size = page_size.max(1);
        let total_items = Self::filter(filter.clone()).count().get_result(db)?;
        let items = Self::filter(filter).limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// A utility function to help build custom search queries
    /// 
    /// Example:
    /// 
    /// ```
    /// // create a filter for completed todos
    /// let query = Todo::filter(TodoFilter {
    ///     completed: Some(true),
    ///     ..Default::default()
    /// });
    /// 
    /// // delete completed todos
    /// diesel::delete(query).execute(db)?;
    /// ```
    pub fn filter<'a>(
        filter: TodosFilter,
    ) -> crate::data::schema::todos::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::data::schema::todos::table.into_boxed();
        
        if let Some(filter_id) = filter.id.clone() {
            query = query.filter(crate::data::schema::todos::id.eq(filter_id));
        }
        if let Some(filter_unsigned) = filter.unsigned.clone() {
            query = query.filter(crate::data::schema::todos::unsigned.eq(filter_unsigned));
        }
        if let Some(filter_text) = filter.text.clone() {
            query = query.filter(crate::data::schema::todos::text.eq(filter_text));
        }
        if let Some(filter_completed) = filter.completed.clone() {
            query = query.filter(crate::data::schema::todos::completed.eq(filter_completed));
        }
        if let Some(filter_created_at) = filter.created_at.clone() {
            query = query.filter(crate::data::schema::todos::created_at.eq(filter_created_at));
        }
        if let Some(filter_updated_at) = filter.updated_at.clone() {
            query = query.filter(crate::data::schema::todos::updated_at.eq(filter_updated_at));
        }
        
        query
    }

    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateTodos) -> QueryResult<Self> {
        use crate::data::schema::todos::dsl::*;

        diesel::update(todos.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::data::schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(param_id))).execute(db)
    }

}
#[derive(Debug, Default, Clone)]
pub struct TodosFilter {
    pub id: Option<i32>,
    pub unsigned: Option<u32>,
    pub text: Option<String>,
    pub completed: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
