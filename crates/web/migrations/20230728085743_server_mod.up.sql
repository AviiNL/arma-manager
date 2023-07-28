-- Add up migration script here
-- Alter table preset_items to add a column "server_mod: bool"

ALTER TABLE preset_items ADD COLUMN server_mod BOOL NOT NULL DEFAULT 0;