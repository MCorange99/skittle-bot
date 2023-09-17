-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "core_users" (
	"user_id"			INTEGER,
	"user_name"			TEXT NOT NULL,
	"user_nickname"		TEXT NOT NULL,
	"user_created_at"	INTEGER NOT NULL,
	"user_role"			INTEGER NOT NULL,
	PRIMARY KEY("user_id")
);