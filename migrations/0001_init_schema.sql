-- Migration number: 0001 	 2026-07-12T11:20:36.808Z
CREATE TABLE users (
                       id INTEGER PRIMARY KEY AUTOINCREMENT,
                       phone TEXT UNIQUE NOT NULL,
                       name TEXT NOT NULL,
                       created_at TEXT NOT NULL
                           DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);