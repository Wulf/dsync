/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=todos, primary_key(text))]
pub struct Todos {
    pub text: String,
    pub text_nullable: Option<String>,
    pub varchar: String,
    pub varchar_nullable: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct CreateTodos<'a> {
    pub text: Cow<'a, str>,
    pub text_nullable: Option<Cow<'a, str>>,
    pub varchar: Cow<'a, str>,
    pub varchar_nullable: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodos<'a> {
    pub text_nullable: Option<Option<Cow<'a, str>>>,
    pub varchar: Option<Cow<'a, str>>,
    pub varchar_nullable: Option<Option<Cow<'a, str>>>,
}


#[derive(Debug, serde::Serialize)]
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
        use crate::schema::todos::dsl::*;

        insert_into(todos).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_text: String) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        todos.filter(text.eq(param_text)).first::<Self>(db)
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
        
        
        if let Some(filter_text) = filter.text.clone() {
            query = query.filter(crate::schema::todos::text.eq(filter_text));
        }
        if let Some(filter_text_nullable) = filter.text_nullable.clone() {
            query = query.filter(crate::schema::todos::text_nullable.eq(filter_text_nullable));
        }
        if let Some(filter_varchar) = filter.varchar.clone() {
            query = query.filter(crate::schema::todos::varchar.eq(filter_varchar));
        }
        if let Some(filter_varchar_nullable) = filter.varchar_nullable.clone() {
            query = query.filter(crate::schema::todos::varchar_nullable.eq(filter_varchar_nullable));
        }
        
        query
    }

    pub fn update(db: &mut ConnectionType, param_text: String, item: &UpdateTodos) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        diesel::update(todos.filter(text.eq(param_text))).set(item).get_result(db)
    }

    pub fn delete(db: &mut ConnectionType, param_text: String) -> QueryResult<usize> {
        use crate::schema::todos::dsl::*;

        diesel::delete(todos.filter(text.eq(param_text))).execute(db)
    }

}
#[derive(Clone)]
pub struct TodosFilter {
    pub text: Option<String>,
    pub text_nullable: Option<Option<String>>,
    pub varchar: Option<String>,
    pub varchar_nullable: Option<Option<String>>,
}

impl Default for TodosFilter {
    fn default() -> TodosFilter {
        TodosFilter {
            text: None,
            text_nullable: None,
            varchar: None,
            varchar_nullable: None,
        }
    }
}
