/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};

type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct UpdateTodo {
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct CreateTodo {
    pub text: String,
    pub completed: bool,
}

pub fn create(db: &mut Connection, item: &CreateTodo) -> QueryResult<Todo> {
    use crate::schema::todos::dsl::*;

    insert_into(todos).values(item).get_result::<Todo>(db)
}

pub fn read(db: &mut Connection, param_id: i32) -> QueryResult<Todo> {
    use crate::schema::todos::dsl::*;

    todos.filter(id.eq(param_id)).first::<Todo>(db)
}

pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<Vec<Todo>> {
    use crate::schema::todos::dsl::*;

    todos.limit(page_size).offset(page * page_size).load::<Todo>(db)
}

pub fn update(db: &mut Connection, param_id: i32, item: &UpdateTodo) -> QueryResult<Todo> {
    use crate::schema::todos::dsl::*;

    diesel::update(todos.filter(id.eq(param_id))).set(item).get_result(db)
}

pub fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
    use crate::schema::todos::dsl::*;

    diesel::delete(todos.filter(id.eq(param_id))).execute(db)
}
