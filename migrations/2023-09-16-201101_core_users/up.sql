-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "core_users" (
	"user_id"			BIGINT,
	"user_name"			TEXT NOT NULL,
	"user_nickname"		TEXT NOT NULL,
	"user_created_at"	BIGINT NOT NULL,
	"user_role"			BIGINT NOT NULL,
	PRIMARY KEY("user_id")
);