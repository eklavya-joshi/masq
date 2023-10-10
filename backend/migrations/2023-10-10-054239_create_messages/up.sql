-- Your SQL goes here
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    author UUID, 
        FOREIGN KEY(author) REFERENCES Users(id),
    content TEXT NOT NULL,
    created TIMESTAMP NOT NULL
)