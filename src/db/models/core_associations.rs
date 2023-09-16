use diesel::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::core_associations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct core_associations {
    pub association_id: i64,
    pub user_id: i64,
    pub associated_user_id: i64,
}