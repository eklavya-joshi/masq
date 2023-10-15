-- Add up migration script here
CREATE TABLE Inbox (
    id UUID PRIMARY KEY NOT NULL,
    created TIMESTAMP NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
)