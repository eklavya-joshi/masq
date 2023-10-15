-- Add up migration script here
CREATE TABLE InboxDmRecipients (
    inbox UUID NOT NULL,
        FOREIGN KEY(inbox) REFERENCES Inbox(id),
    recipient1 UUID NOT NULL,
        FOREIGN KEY(recipient1) REFERENCES Users(id),
    recipient2 UUID NOT NULL,
        FOREIGN KEY(recipient2) REFERENCES Users(id)
)