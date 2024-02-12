/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;
use crate::models::common::*;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Struct representing a row in table `table1`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=table1, primary_key(id))]
pub struct Table1 {
    /// Field representing column `id`
    pub id: i32,
}

impl Table1 {
    /// Insert a new row into `table1` with all default values
    pub fn create(db: &mut ConnectionType) -> diesel::QueryResult<Self> {
        use crate::schema::table1::dsl::*;

        diesel::insert_into(table1).default_values().get_result::<Self>(db)
    }

    /// Get a row from `table1`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::table1::dsl::*;

        table1.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Delete a row in `table1`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::table1::dsl::*;

        diesel::delete(table1.filter(id.eq(param_id))).execute(db)
    }
}
