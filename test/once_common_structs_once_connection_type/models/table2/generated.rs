/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;
use crate::models::common::*;

/// Struct representing a row in table `table2`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=table2, primary_key(id))]
pub struct Table2 {
    /// Field representing column `id`
    pub id: i32,
}

impl Table2 {
    /// Insert a new row into `table2` with all default values
    pub fn create(db: &mut ConnectionType) -> diesel::QueryResult<Self> {
        use crate::schema::table2::dsl::*;

        diesel::insert_into(table2).default_values().get_result::<Self>(db)
    }

    /// Get a row from `table2`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::table2::dsl::*;

        table2.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Delete a row in `table2`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::table2::dsl::*;

        diesel::delete(table2.filter(id.eq(param_id))).execute(db)
    }
}
