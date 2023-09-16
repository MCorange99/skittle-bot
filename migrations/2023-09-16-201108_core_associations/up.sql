-- Your SQL goes here
CREATE TABLE core_associations (
    association_id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    associated_user_id INTEGER NOT NULL,
    FOREIGN KEY(user_id) REFERENCES core_users(user_id)
);