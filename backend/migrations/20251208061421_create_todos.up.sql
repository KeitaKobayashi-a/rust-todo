-- Add migration script here
CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    title TEXT NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'Asia/Tokyo') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'Asia/Tokyo') NOT NULL
);

INSERT INTO todos(title, description) VALUES(
    'wake up 6 AM', 'this is sample text'
),(
    'eat breakfast', 'this is sample text'
),(
    'get ready to go out', 'this is sample text'
),(
    'get home', 'this is sample text'
)