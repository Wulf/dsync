/* @generated and managed by dsync */

use crate::diesel::*;
use crate::data::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name=tableA, primary_key(_id))]
pub struct TableA {
    pub _id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable)]
#[diesel(table_name=tableA)]
pub struct CreateTableA {
    pub _id: i32,
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

impl TableA {

    pub fn create(db: &mut ConnectionType, item: &CreateTableA) -> QueryResult<Self> {
        use crate::data::schema::tableA::dsl::*;

        insert_into(tableA).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param__id: i32) -> QueryResult<Self> {
        use crate::data::schema::tableA::dsl::*;

        tableA.filter(_id.eq(param__id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: TableAFilter) -> QueryResult<PaginationResult<Self>> {
        use crate::data::schema::tableA::dsl::*;

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
        filter: TableAFilter,
    ) -> crate::data::schema::tableA::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::data::schema::tableA::table.into_boxed();
        
        if let Some(filter__id) = filter._id.clone() {
            query = query.filter(crate::data::schema::tableA::_id.eq(filter__id));
        }
        
        query
    }

    pub fn delete(db: &mut ConnectionType, param__id: i32) -> QueryResult<usize> {
        use crate::data::schema::tableA::dsl::*;

        diesel::delete(tableA.filter(_id.eq(param__id))).execute(db)
    }

}
#[derive(Debug, Default, Clone)]
pub struct TableAFilter {
    pub _id: Option<i32>,
}
