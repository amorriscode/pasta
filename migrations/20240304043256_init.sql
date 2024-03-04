CREATE TABLE IF NOT EXISTS clipboard
(
    id          INTEGER PRIMARY KEY NOT NULL,
    content     TEXT                NOT NULL,
    created_at  DATETIME            NOT NULL DEFAULT (datetime('now'))
);
