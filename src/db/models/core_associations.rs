use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::core_associations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CoreAssociations {
    pub association_id: i64,
    pub user_id: i64,
    pub associated_user_id: i64,
}