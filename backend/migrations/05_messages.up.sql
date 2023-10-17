-- Add up migration script here
CREATE TABLE Messages (
    id UUID PRIMARY KEY NOT NULL,
    author_id UUID NOT NULL, 
        FOREIGN KEY(author_id) REFERENCES Users(id),
    author_name VARCHAR(50) NOT NULL,
    inbox UUID NOT NULL, 
        FOREIGN KEY(inbox) REFERENCES Inbox(id),
    content TEXT NOT NULL,
    created TIMESTAMP NOT NULL
)