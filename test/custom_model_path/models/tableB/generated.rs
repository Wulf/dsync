/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::data::models::table_a::TableA;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

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

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=tableB)]
pub struct UpdateTableB {
    pub link: Option<i32>,
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

impl TableB {

    pub fn create(db: &mut ConnectionType, item: &CreateTableB) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        insert_into(tableB).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param__id: i32) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        tableB.filter(_id.eq(param__id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 1-based index (i.e. page 1 is the first page)
    pub fn paginate(db: &mut ConnectionType, param_page_starting_with_1: i64, param_page_size: i64, filter: TableBFilter) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::tableB::dsl::*;

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
        filter: TableBFilter,
    ) -> crate::schema::tableB::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::tableB::table.into_boxed();
        
        
        if let Some(filter__id) = filter._id.clone() {
            query = query.filter(crate::schema::tableB::_id.eq(filter__id));
        }
        if let Some(filter_link) = filter.link.clone() {
            query = query.filter(crate::schema::tableB::link.eq(filter_link));
        }
        
        query
    }

    pub fn update(db: &mut ConnectionType, param__id: i32, item: &UpdateTableB) -> QueryResult<Self> {
        use crate::schema::tableB::dsl::*;

        diesel::update(tableB.filter(_id.eq(param__id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut ConnectionType, param__id: i32) -> QueryResult<usize> {
        use crate::schema::tableB::dsl::*;

        diesel::delete(tableB.filter(_id.eq(param__id))).execute(db)
    }

}
#[derive(Clone)]
pub struct TableBFilter {
    pub _id: Option<i32>,
    pub link: Option<i32>,
}

impl Default for TableBFilter {
    fn default() -> TableBFilter {
        TableBFilter {
            _id: None,
            link: None,
        }
    }
}
