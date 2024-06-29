-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
    id bigserial PRIMARY KEY,
    fullname varchar(64) NOT NULL,
    email varchar(64) NOT NULL,
    password_hash varchar(97) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

-- index
CREATE UNIQUE INDEX IF NOT EXISTS email_index ON users(email);

-- table chat_type
CREATE TYPE chat_type AS ENUM(
    'single',
    'group',
    'private_channel',
    'public_channel'
);

-- table: chat
CREATE TABLE IF NOT EXISTS chats(
    id bigserial PRIMARY KEY,
    name varchar(128) NOT NULL UNIQUE,
    type chat_type NOT NULL,
    members bigint[] NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

-- table: message
CREATE TABLE IF NOT EXISTS messages(
    id bigserial PRIMARY KEY,
    chat_id bigint NOT NULL REFERENCES chats(id),
    sender_id bigint NOT NULL REFERENCES users(id),
    content text NOT NULL,
    images text[],
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

-- message index for: chat_id
CREATE INDEX IF NOT EXISTS chat_id_created_at_index ON messages(chat_id, created_at DESC);

-- message index for:  sender_id
CREATE INDEX IF NOT EXISTS sender_id_index ON messages(sender_id, created_at DESC);
