/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todos {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodos {
    pub created_at: Option<chrono::NaiveDateTime>,
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

    pub fn create(db: &mut ConnectionType) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        insert_into(todos).default_values().get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        todos.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 1-based index (i.e. page 1 is the first page)
    pub fn paginate(db: &mut ConnectionType, param_page_starting_with_1: i64, param_page_size: i64, filter: TodosFilter) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::todos::dsl::*;

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
        filter: TodosFilter,
    ) -> crate::schema::todos::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::todos::table.into_boxed();
        
        
        if let Some(filter_id) = filter.id.clone() {
            query = query.filter(crate::schema::todos::id.eq(filter_id));
        }
        if let Some(filter_created_at) = filter.created_at.clone() {
            query = query.filter(crate::schema::todos::created_at.eq(filter_created_at));
        }
        
        query
    }

    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateTodos) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        diesel::update(todos.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(param_id))).execute(db)
    }

}
#[derive(Clone)]
pub struct TodosFilter {
    pub id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Default for TodosFilter {
    fn default() -> TodosFilter {
        TodosFilter {
            id: None,
            created_at: None,
        }
    }
}
