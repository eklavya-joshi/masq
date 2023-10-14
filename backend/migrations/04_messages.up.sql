-- Add up migration script here
CREATE TABLE Messages (
    id UUID PRIMARY KEY,
    author UUID NOT NULL, 
        FOREIGN KEY(author) REFERENCES Users(id),
    inbox UUID NOT NULL, 
        FOREIGN KEY(inbox) REFERENCES Inbox(id),
    content TEXT NOT NULL,
    created TIMESTAMP NOT NULL
)