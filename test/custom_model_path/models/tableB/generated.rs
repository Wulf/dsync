/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::data::models::table_a::TableA;

type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=tableB, primary_key(_id), belongs_to(TableA, foreign_key=link))]
pub struct TableB {
    pub _id: i32,
    pub link: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=tableB)]
pub struct CreateTableB {
    pub _id: i32,
    pub link: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=tableB)]
pub struct UpdateTableB {
    pub link: Option<i32>,
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

impl TableB {

    pub fn create(db: &mut Connection, item: &CreateTableB) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        insert_into(tableB).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut Connection, param__id: i32) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        tableB.filter(_id.eq(param__id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::tableB::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = tableB.count().get_result(db)?;
        let items = tableB.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param__id: i32, item: &UpdateTableB) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        diesel::update(tableB.filter(_id.eq(param__id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param__id: i32) -> QueryResult<usize> {
        use crate::schema::tableB::dsl::*;

        diesel::delete(tableB.filter(_id.eq(param__id))).execute(db)
    }

}