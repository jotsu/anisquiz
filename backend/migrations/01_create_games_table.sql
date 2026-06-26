-- Add migration script here
CREATE TABLE IF NOT EXISTS games (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL DEFAULT 'New Game',
    owner_id TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    round INTEGER NOT NULL DEFAULT 1,
    turn INTEGER NOT NULL DEFAULT 1,
    active_team_id TEXT DEFAULT NULL,
    active_quest_id TEXT DEFAULT NULL,

    FOREIGN KEY(owner_id) REFERENCES users(id),
    FOREIGN KEY(active_team_id) REFERENCES teams(id),
    FOREIGN KEY(active_quest_id) REFERENCES quests(id)
);