/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use crate::models::common::*;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name=table1, primary_key(id))]
pub struct Table1 {
    pub id: crate::schema::sql_types::Int,
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

    /// Paginates through the table where page is a 1-based index (i.e. page 1 is the first page)
    pub fn paginate(db: &mut ConnectionType, page_starting_with_1: i64, page_size: i64, filter: Table1Filter) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::table1::dsl::*;

        let param_page = page_starting_with_1.max(0);
        let page_size = page_size.max(1);
        let total_items = Self::filter(filter.clone()).count().get_result(db)?;
        let items = Self::filter(filter).limit(page_size).offset(param_page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page: param_page,
            page_size: page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// A utility function to help build custom search queries
    /// 
    /// Example:
    /// 
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
#[derive(Clone)]
pub struct Table1Filter {
    pub id: Option<crate::schema::sql_types::Int>,
}

impl Default for Table1Filter {
    fn default() -> Table1Filter {
        Table1Filter {
            id: None,
        }
    }
}
