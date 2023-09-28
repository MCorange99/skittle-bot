use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ModerationMessages {
    pub message_id: i64,
    pub message_link: String,
}