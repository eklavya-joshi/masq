-- Your SQL goes here
CREATE TABLE message_recipients (
    id UUID PRIMARY KEY,
    message_id UUID,
        FOREIGN KEY(message_id) REFERENCES Messages(id),
    recipient UUID,
        FOREIGN KEY(recipient) REFERENCES Users(id),
    recipient_group UUID,
        FOREIGN KEY(recipient_group) REFERENCES Groups(id)
)