use diesel::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct moderation_messages {
    pub message_id: i64,
    pub message_link: String,
}