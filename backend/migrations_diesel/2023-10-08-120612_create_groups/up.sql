-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE groups (
    id UUID PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    created TIMESTAMP NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
)