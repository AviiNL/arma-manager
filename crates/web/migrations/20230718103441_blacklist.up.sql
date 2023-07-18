-- Add up migration script here
CREATE TABLE "blacklist" (
    "published_file_id" INTEGER NOT NULL UNIQUE,
    PRIMARY KEY("published_file_id")
);
