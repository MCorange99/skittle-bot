use diesel::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_message_references)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct moderation_message_references {
    pub entry_id: i64,
    pub message_id: i64,
    pub note: Option<String>,
}