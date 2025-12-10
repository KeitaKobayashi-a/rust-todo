-- Add migration script here
CREATE TABLE todos
(
    id          UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    title       TEXT                                                               NOT NULL,
    description TEXT,
    completed   BOOLEAN                                                            NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'Asia/Tokyo') NOT NULL,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'Asia/Tokyo') NOT NULL
);

INSERT INTO todos(title, description)
VALUES ('wake up at 6 AM', 'Start the day early and stretch for a few minutes'),
       ('drink a glass of water', 'Hydrate before coffee'),
       ('eat breakfast', 'Keep it simple and healthy'),
       ('review today''s plan', 'Check top 3 priorities'),
       ('check emails', 'Reply to urgent messages only'),
       ('focus on deep work', 'Work in a 60-minute 집중 session'),
       ('organize the desk', 'Clear clutter and sort notes');

