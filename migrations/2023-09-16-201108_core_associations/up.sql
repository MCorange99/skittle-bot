-- Your SQL goes here
CREATE TABLE core_associations (
    "association_id"		BIGINT NOT NULL,
	"user_id"				BIGINT NOT NULL,
	"associated_user_id"	BIGINT NOT NULL,
	FOREIGN KEY("associated_user_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("user_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("association_id")
);