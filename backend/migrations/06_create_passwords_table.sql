-- Add migration script here
CREATE TABLE IF NOT EXISTS passwords (
    user_id TEXT NOT NULL PRIMARY KEY,
    password_hash TEXT NOT NULL,

    FOREIGN KEY(user_id) REFERENCES users(id)
);