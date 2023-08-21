-- Add up migration script here
CREATE TABLE todos (
    id INTEGER PRIMARY KEY,
    text VARCHAR(255) NOT NULL,
    is_done BOOLEAN NOT NULL DEFAULT FALSE
);