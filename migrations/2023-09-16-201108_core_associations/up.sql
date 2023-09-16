-- Your SQL goes here
CREATE TABLE core_associations (
    "association_id"	INTEGER,
	"user_id"	INTEGER NOT NULL,
	"associated_user_id"	INTEGER NOT NULL,
	FOREIGN KEY("associated_user_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("user_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("association_id")
);