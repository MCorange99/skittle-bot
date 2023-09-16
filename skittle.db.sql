BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "users" (
	"user_id"	INTEGER,
	PRIMARY KEY("user_id")
);
CREATE TABLE IF NOT EXISTS "associations" (
	"association_id"	INTEGER,
	"user_id"	INTEGER NOT NULL,
	"associated_user_id"	INTEGER NOT NULL,
	FOREIGN KEY("associated_user_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("user_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("association_id")
);
CREATE TABLE IF NOT EXISTS "mutes" (
	"mute_id"	INTEGER,
	"target_id"	INTEGER NOT NULL,
	"moderator_id"	INTEGER NOT NULL,
	"reason"	TEXT,
	"timestamp"	NUMERIC NOT NULL,
	"duration"	NUMERIC,
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("mute_id")
);
CREATE TABLE IF NOT EXISTS "user_reports" (
	"report_id"	INTEGER,
	"target_id"	INTEGER NOT NULL,
	"moderator_id"	INTEGER NOT NULL,
	"report_title"	TEXT,
	"report_description"	TEXT,
	"timestamp"	NUMERIC,
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("report_id")
);
CREATE TABLE IF NOT EXISTS "messages" (
	"message_id"	INTEGER,
	"message_link"	TEXT NOT NULL,
	PRIMARY KEY("message_id")
);
CREATE TABLE IF NOT EXISTS "message_references" (
	"message_reference"	INTEGER,
	"entry_id"	INTEGER,
	"message_id"	INTEGER NOT NULL,
	"note"	TEXT,
	FOREIGN KEY("message_id") REFERENCES "messages"("message_id"),
	PRIMARY KEY("message_reference","entry_id")
);
CREATE TABLE IF NOT EXISTS "kicks" (
	"kick_id"	INTEGER,
	"target_id"	INTEGER NOT NULL,
	"moderator_id"	INTEGER NOT NULL,
	"kick_reason"	TEXT,
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("kick_id")
);
CREATE TABLE IF NOT EXISTS "bans" (
	"ban_id"	INTEGER,
	"target_id"	INTEGER NOT NULL,
	"moderator_id"	INTEGER NOT NULL,
	"ban_reason"	TEXT,
	"ban_duration"	NUMERIC,
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("ban_id")
);
CREATE TABLE IF NOT EXISTS "roles" (
	"role_name"	TEXT,
	PRIMARY KEY("role_name")
);
CREATE TABLE IF NOT EXISTS "reacts_for_roles" (
	"reaction_name"	TEXT,
	"role"	TEXT NOT NULL,
	FOREIGN KEY("role") REFERENCES "roles"("role_name"),
	PRIMARY KEY("reaction_name")
);
CREATE TABLE IF NOT EXISTS "react_messages" (
	"message_id"	INTEGER,
	"role_name"	TEXT,
	FOREIGN KEY("message_id") REFERENCES "messages"("message_id"),
	FOREIGN KEY("role_name") REFERENCES "roles"("role_name"),
	PRIMARY KEY("message_id","role_name")
);
CREATE TABLE IF NOT EXISTS "react_roles" (
	"react_message"	INTEGER,
	"role_name"	TEXT,
	"user_id"	INTEGER,
	FOREIGN KEY("react_message") REFERENCES "react_messages"("message_id"),
	FOREIGN KEY("role_name") REFERENCES "react_messages"("role_name"),
	FOREIGN KEY("user_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("react_message","role_name","user_id")
);
CREATE TABLE IF NOT EXISTS "user_notes" (
	"note_id"	INTEGER,
	"target_id"	INTEGER NOT NULL,
	"moderator_id"	INTEGER NOT NULL,
	"note"	TEXT NOT NULL,
	"message_reference"	INTEGER,
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("message_reference") REFERENCES "message_references"("message_reference"),
	PRIMARY KEY("note_id")
);
COMMIT;
