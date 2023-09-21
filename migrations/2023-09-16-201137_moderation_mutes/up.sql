-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_mutes" (
	"mute_id"	BIGINT,
	"target_id"	BIGINT NOT NULL,
	"moderator_id"	BIGINT NOT NULL,
	"reason"	TEXT,
	"timestamp"	NUMERIC NOT NULL,
	"duration"	NUMERIC,
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
    PRIMARY KEY("mute_id")
);