-- Your SQL goes here
CREATE TABLE notebooks (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    notebook_id INTEGER REFERENCES notebooks(id) NOT NULL
);
