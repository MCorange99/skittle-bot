-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_reports" (
	"report_id"	INTEGER,
	"target_id"	INTEGER NOT NULL,
	"moderator_id"	INTEGER NOT NULL,
	"report_title"	TEXT,
	"report_description"	TEXT,
	"timestamp"	NUMERIC,
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id")
    PRIMARY KEY("report_id")
);