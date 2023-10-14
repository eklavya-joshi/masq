-- Add up migration script here
CREATE TABLE InboxRecipients (
    inbox UUID ,
        FOREIGN KEY(inbox) REFERENCES Inbox(id),
    recipient UUID,
        FOREIGN KEY(recipient) REFERENCES Users(id)
)