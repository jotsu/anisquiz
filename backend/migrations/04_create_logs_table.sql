-- Add migration script here
CREATE TABLE IF NOT EXISTS logs (
    id TEXT PRIMARY KEY NOT NULL,
    parent_game_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    message text NOT NULL,
    
    FOREIGN KEY(parent_game_id) REFERENCES games(id)
);