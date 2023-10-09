/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=tableA, primary_key(_id))]
pub struct TableA {
    pub _id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
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
        use crate::schema::tableA::dsl::*;

        insert_into(tableA).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param__id: i32) -> QueryResult<Self> {
        use crate::schema::tableA::dsl::*;

        tableA.filter(_id.eq(param__id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::tableA::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = tableA.count().get_result(db)?;
        let items = tableA.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn delete(db: &mut ConnectionType, param__id: i32) -> QueryResult<usize> {
        use crate::schema::tableA::dsl::*;

        diesel::delete(tableA.filter(_id.eq(param__id))).execute(db)
    }

}