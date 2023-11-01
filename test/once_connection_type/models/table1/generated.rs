/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use crate::models::common::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name=table1, primary_key(id))]
pub struct Table1 {
    pub id: crate::schema::sql_types::Int,
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

impl Table1 {

    pub fn create(db: &mut ConnectionType) -> QueryResult<Self> {
        use crate::schema::table1::dsl::*;

        insert_into(table1).default_values().get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_id: crate::schema::sql_types::Int) -> QueryResult<Self> {
        use crate::schema::table1::dsl::*;

        table1.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: Table1Filter) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::table1::dsl::*;

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
        filter: Table1Filter,
    ) -> crate::schema::table1::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::table1::table.into_boxed();
        
        if let Some(filter_id) = filter.id.clone() {
            query = query.filter(crate::schema::table1::id.eq(filter_id));
        }
        
        query
    }

    pub fn delete(db: &mut ConnectionType, param_id: crate::schema::sql_types::Int) -> QueryResult<usize> {
        use crate::schema::table1::dsl::*;

        diesel::delete(table1.filter(id.eq(param_id))).execute(db)
    }

}
#[derive(Debug, Default, Clone)]
pub struct Table1Filter {
    pub id: Option<crate::schema::sql_types::Int>,
}
