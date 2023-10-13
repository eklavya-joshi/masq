-- Add up migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    salt VARCHAR(150),
    pass VARCHAR(150) NOT NULL,
    created TIMESTAMP NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    token TEXT
)