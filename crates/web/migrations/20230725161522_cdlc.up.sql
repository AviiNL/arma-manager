-- Add up migration script here
-- CREATE TABLE "dlc" (
--     "id"                INTEGER NOT NULL UNIQUE,
--     "name"              TEXT NOT NULL,
--     "key"               TEXT NOT NULL,
--     "app_id"            INTEGER NOT NULL,
--     PRIMARY KEY("id" AUTOINCREMENT)
-- );

-- INSERT INTO "dlc" (id, name, key, app_id) VALUES
--     (1, "Spearhead 1944", "spe", 1175380),
--     (2, "Western Sahara", "ws", 1681170),
--     (3, "S.O.G. Prairie Fire", "vn", 1227700),
--     (4, "CSLA Iron Curtain", "csla", 1294440),
--     (5, "Global Mobilization", "gm", 1042220),
--     (6, "Contact", "enoch", 1021790);

CREATE TABLE "preset_dlc" (
    "id"                INTEGER NOT NULL UNIQUE,
    "preset_id"         INTEGER NOT NULL,
    "name"              TEXT NOT NULL,
    "key"               TEXT NOT NULL,
    "app_id"            INTEGER NOT NULL,
    "position"          INTEGER NOT NULL,
    "enabled"           BOOL NOT NULL DEFAULT 1,
    "created_at"        TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at"        TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY("id" AUTOINCREMENT)
    FOREIGN KEY("preset_id") REFERENCES "presets"("id")
);