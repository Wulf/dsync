/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name=table2, primary_key(id))]
pub struct Table2 {
    pub id: crate::schema::sql_types::Int,
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

impl Table2 {

    pub fn create(db: &mut ConnectionType) -> QueryResult<Self> {
        use crate::schema::table2::dsl::*;

        insert_into(table2).default_values().get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_id: crate::schema::sql_types::Int) -> QueryResult<Self> {
        use crate::schema::table2::dsl::*;

        table2.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 1-based index (i.e. page 1 is the first page)
    pub fn paginate(db: &mut ConnectionType, param_page_starting_with_1: i64, param_page_size: i64, filter: Table2Filter) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::table2::dsl::*;

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
        filter: Table2Filter,
    ) -> crate::schema::table2::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::table2::table.into_boxed();
        
        
        if let Some(filter_id) = filter.id.clone() {
            query = query.filter(crate::schema::table2::id.eq(filter_id));
        }
        
        query
    }

    pub fn delete(db: &mut ConnectionType, param_id: crate::schema::sql_types::Int) -> QueryResult<usize> {
        use crate::schema::table2::dsl::*;

        diesel::delete(table2.filter(id.eq(param_id))).execute(db)
    }

}
#[derive(Clone)]
pub struct Table2Filter {
    pub id: Option<crate::schema::sql_types::Int>,
}

impl Default for Table2Filter {
    fn default() -> Table2Filter {
        Table2Filter {
            id: None,
        }
    }
}
