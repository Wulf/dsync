/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Struct representing a row in table `todos`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todos {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `unsigned`
    pub unsigned: u32,
    /// Field representing column `unsigned_nullable`
    pub unsigned_nullable: Option<u32>,
    /// Field representing column `text`
    pub text: String,
    /// Field representing column `completed`
    pub completed: bool,
    /// Field representing column `type`
    pub type_: String,
    /// Field representing column `created_at`
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Field representing column `updated_at`
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Create Struct for a row in table `todos` for [`Todos`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=todos)]
pub struct CreateTodos {
    /// Field representing column `unsigned`
    pub unsigned: u32,
    /// Field representing column `unsigned_nullable`
    pub unsigned_nullable: Option<u32>,
    /// Field representing column `text`
    pub text: String,
    /// Field representing column `completed`
    pub completed: bool,
    /// Field representing column `type`
    pub type_: String,
}

/// Update Struct for a row in table `todos` for [`Todos`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodos {
    /// Field representing column `unsigned`
    pub unsigned: Option<u32>,
    /// Field representing column `unsigned_nullable`
    pub unsigned_nullable: Option<Option<u32>>,
    /// Field representing column `text`
    pub text: Option<String>,
    /// Field representing column `completed`
    pub completed: Option<bool>,
    /// Field representing column `type`
    pub type_: Option<String>,
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

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: TodosFilter) -> diesel::QueryResult<PaginationResult<Self>> {
        use crate::schema::todos::dsl::*;

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
    ) -> crate::schema::todos::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::todos::table.into_boxed();
        
        if let Some(filter_id) = filter.id.clone() {
            query = query.filter(crate::schema::todos::id.eq(filter_id));
        }
        if let Some(filter_unsigned) = filter.unsigned.clone() {
            query = query.filter(crate::schema::todos::unsigned.eq(filter_unsigned));
        }
        if let Some(filter_unsigned_nullable) = filter.unsigned_nullable.clone() {
            query = query.filter(crate::schema::todos::unsigned_nullable.eq(filter_unsigned_nullable));
        }
        if let Some(filter_text) = filter.text.clone() {
            query = query.filter(crate::schema::todos::text.eq(filter_text));
        }
        if let Some(filter_completed) = filter.completed.clone() {
            query = query.filter(crate::schema::todos::completed.eq(filter_completed));
        }
        if let Some(filter_type_) = filter.type_.clone() {
            query = query.filter(crate::schema::todos::type_.eq(filter_type_));
        }
        if let Some(filter_created_at) = filter.created_at.clone() {
            query = query.filter(crate::schema::todos::created_at.eq(filter_created_at));
        }
        if let Some(filter_updated_at) = filter.updated_at.clone() {
            query = query.filter(crate::schema::todos::updated_at.eq(filter_updated_at));
        }
        
        query
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

    #[derive(Debug, Default, Clone)]
    pub struct TodosFilter {
        pub id: Option<i32>,
    pub unsigned: Option<u32>,
    pub unsigned_nullable: Option<Option<u32>>,
    pub text: Option<String>,
    pub completed: Option<bool>,
    pub type_: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    }
    