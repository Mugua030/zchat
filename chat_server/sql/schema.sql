CREATE TABLE IF NOT EXISTS users {
    id bigserial PRIMARY KEY,
    fullname varchar(64) NOT NULL,
    email varchar(64) NOT NULL,
    password VARCHAR(64) NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP
};

CREATE UNIQUE INDEX IF NOT EXISTS email_index ON users(email);

CREATE TYPE chat_type AS ENUM {
    'single',
    'group',
    'private_channel',
    'public_channel',
};

CREATE TABLE IF NOT EXISTS chats {
    id bigserial PRIMARY KEY,
    name VARCHAR(128) NOT NULL UNIQUE,
    type chat_type NOT NULL,
    members bigint[] NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP
};

CREATE TABLE IF NOT EXISTS messages {
    id bigserial PRIMARY KEY,
    chat_id bigint NOT NULL,
    sender_id bigint NOT NULL,
    content TEXT NOT NULL,
    images TEXT[],
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(chat_id) REFERENCES chats(id),
    FOREIGN Key(sender_id) REFERENCES users(id),
}

CREATE INDEX IF NOT EXISTS chat_id_created_at_index ON messages(chat_id, created_at DESC);

CREATE INDEX IF NOT EXISTS sender_id_index ON messages(sender_id);
