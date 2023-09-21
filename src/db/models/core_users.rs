use diesel::prelude::*;
use diesel::sql_types::BigInt;

#[allow(non_camel_case_types)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::core_users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct core_users {
    #[diesel(sql_type = BigInt)]
    pub user_id: i64,
}

#[allow(non_camel_case_types)]
#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::core_users)]
pub struct new_core_users {
    #[diesel(sql_type = BigInt)]
    pub user_id: i64,
}

// impl core_users {
//     pub fn create(&self, conn)
// }