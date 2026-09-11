-- Your SQL goes here
CREATE TABLE expenses (
    id TEXT PRIMARY KEY NOT NULL,
    expense_type TEXT NOT NULL,
    amount REAL NOT NULL,
    date DATE NOT NULL,
    description TEXT
);