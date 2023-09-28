-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_bans" (
	"ban_id"		BIGINT NOT NULL,
	"target_id"		BIGINT NOT NULL,
	"moderator_id"	BIGINT NOT NULL,
	"ban_reason"	TEXT,
	"ban_duration"	BIGINT,
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("ban_id")
);