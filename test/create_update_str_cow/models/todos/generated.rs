/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use serde::{Deserialize, Serialize};
use diesel::QueryResult;

type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/// Struct representing a row in table `todos`
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name=todos, primary_key(text))]
pub struct Todos {
    /// Field representing column `text`
    pub text: String,
    /// Field representing column `text_nullable`
    pub text_nullable: Option<String>,
    /// Field representing column `varchar`
    pub varchar: String,
    /// Field representing column `varchar_nullable`
    pub varchar_nullable: Option<String>,
}

/// Create Struct for a row in table `todos` for [`Todos`]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name=todos)]
pub struct CreateTodos<'a> {
    /// Field representing column `text`
    pub text: Cow<'a, str>,
    /// Field representing column `text_nullable`
    pub text_nullable: Option<Cow<'a, str>>,
    /// Field representing column `varchar`
    pub varchar: Cow<'a, str>,
    /// Field representing column `varchar_nullable`
    pub varchar_nullable: Option<Cow<'a, str>>,
}

/// Update Struct for a row in table `todos` for [`Todos`]
#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, PartialEq, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodos<'a> {
    /// Field representing column `text_nullable`
    pub text_nullable: Option<Option<Cow<'a, str>>>,
    /// Field representing column `varchar`
    pub varchar: Option<Cow<'a, str>>,
    /// Field representing column `varchar_nullable`
    pub varchar_nullable: Option<Option<Cow<'a, str>>>,
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

impl Todos {
    /// Insert a new row into `todos` with a given [`CreateTodos`]
    pub fn create(db: &mut ConnectionType, item: &CreateTodos) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        insert_into(todos).values(item).get_result::<Self>(db)
    }

    /// Get a row from `todos`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_text: String) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        todos.filter(text.eq(param_text)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::todos::dsl::*;

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
    pub fn update(db: &mut ConnectionType, param_text: String, item: &UpdateTodos) -> QueryResult<Self> {
        use crate::schema::todos::dsl::*;

        diesel::update(todos.filter(text.eq(param_text))).set(item).get_result(db)
    }

    /// Delete a row in `todos`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_text: String) -> QueryResult<usize> {
        use crate::schema::todos::dsl::*;

        diesel::delete(todos.filter(text.eq(param_text))).execute(db)
    }
}
