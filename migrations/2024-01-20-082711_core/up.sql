-- Your SQL goes here
CREATE TABLE notebooks (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    notebook_id INTEGER REFERENCES notebooks(id) NOT NULL,
    content TEXT NOT NULL
);
