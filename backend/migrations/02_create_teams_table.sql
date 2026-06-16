-- Add migration script here
CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY NOT NULL,
    parent_game_id TEXT NOT NULL,
    no INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL DEFAULT 'New Team',
    score INTEGER NOT NULL DEFAULT 0,
    
    FOREIGN KEY(parent_game_id) REFERENCES games(id)
);