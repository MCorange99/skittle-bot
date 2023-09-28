-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "core_users" (
	"user_id"			BIGINT NOT NULL,
	"user_role"			BIGINT NOT NULL,
	PRIMARY KEY("user_id")
);