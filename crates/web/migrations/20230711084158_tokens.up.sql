-- Add up migration script here
CREATE TABLE "user_tokens" (
    "token"	        UUID NOT NULL,
    "user_id"	    UUID NOT NULL,
    "ip"            TEXT NOT NULL,
	"created_at"	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"last_used"	    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY("token")
    FOREIGN KEY("user_id") REFERENCES "users"("id")
);