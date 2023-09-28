-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_reports" (
	"report_id"	BIGINT NOT NULL,
	"target_id"	BIGINT NOT NULL,
	"moderator_id"	BIGINT NOT NULL,
	"report_title"	TEXT NOT NULL,
	"report_description"	TEXT NOT NULL,
	"timestamp"	BIGINT NOT NULL,
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id")
    PRIMARY KEY("report_id")
);