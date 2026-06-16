-- Add migration script here
CREATE TABLE IF NOT EXISTS quests (
    id TEXT PRIMARY KEY NOT NULL,
    parent_game_id TEXT NOT NULL,
    src TEXT NOT NULL,
    pts INTEGER NOT NULL DEFAULT 3,
    
    FOREIGN KEY(parent_game_id) REFERENCES games(id)
);