-- Add up migration script here
CREATE TABLE InboxRecipients (
    inbox UUID NOT NULL,
        FOREIGN KEY(inbox) REFERENCES Inbox(id),
    recipient UUID NOT NULL,
        FOREIGN KEY(recipient) REFERENCES Users(id)
)