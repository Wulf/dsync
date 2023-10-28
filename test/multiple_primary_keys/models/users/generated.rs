/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=users, primary_key(name,address))]
pub struct Users {
    pub name: String,
    pub address: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=users)]
pub struct CreateUsers {
    pub name: String,
    pub address: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=users)]
pub struct UpdateUsers {
    pub secret: Option<String>,
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

impl Users {

    pub fn create(db: &mut ConnectionType, item: &CreateUsers) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        insert_into(users).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_name: String, param_address: String) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        users.filter(name.eq(param_name)).filter(address.eq(param_address)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 1-based index (i.e. page 1 is the first page)
    pub fn paginate(db: &mut ConnectionType, param_page_starting_with_1: i64, param_page_size: i64, filter: UsersFilter) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::users::dsl::*;

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
        filter: UsersFilter,
    ) -> crate::schema::users::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::users::table.into_boxed();
        
        
        if let Some(filter_name) = filter.name.clone() {
            query = query.filter(crate::schema::users::name.eq(filter_name));
        }
        if let Some(filter_address) = filter.address.clone() {
            query = query.filter(crate::schema::users::address.eq(filter_address));
        }
        if let Some(filter_secret) = filter.secret.clone() {
            query = query.filter(crate::schema::users::secret.eq(filter_secret));
        }
        
        query
    }

    pub fn update(db: &mut ConnectionType, param_name: String, param_address: String, item: &UpdateUsers) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        diesel::update(users.filter(name.eq(param_name)).filter(address.eq(param_address))).set(item).get_result(db)
    }

    pub fn delete(db: &mut ConnectionType, param_name: String, param_address: String) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(name.eq(param_name)).filter(address.eq(param_address))).execute(db)
    }

}
#[derive(Clone)]
pub struct UsersFilter {
    pub name: Option<String>,
    pub address: Option<String>,
    pub secret: Option<String>,
}

impl Default for UsersFilter {
    fn default() -> UsersFilter {
        UsersFilter {
            name: None,
            address: None,
            secret: None,
        }
    }
}
