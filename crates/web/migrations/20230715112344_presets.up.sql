-- Add up migration script here
CREATE TABLE "presets" (
    "id"         INTEGER NOT NULL UNIQUE,
    "name"       TEXT NOT NULL UNIQUE,
    "selected"   BOOL DEFAULT NULL UNIQUE,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "preset_items" (
    "id"                INTEGER NOT NULL UNIQUE,
    "preset_id"         INTEGER NOT NULL,
    "name"              TEXT NOT NULL,
    "published_file_id" INTEGER NOT NULL,
    "position"          INTEGER NOT NULL,
    "enabled"           BOOL NOT NULL DEFAULT 1,
    "created_at"        TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at"        TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY("id" AUTOINCREMENT)
    FOREIGN KEY("preset_id") REFERENCES "presets"("id")
);
