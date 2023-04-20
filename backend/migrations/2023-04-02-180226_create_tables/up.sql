CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    user_name TEXT NOT NULL,
    text TEXT NOT NULL,
    creation_date TIMESTAMP NOT NULL
);

CREATE SEQUENCE user_id;