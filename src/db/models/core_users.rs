use diesel::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::core_users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct core_users {
    pub user_id: i64,
}