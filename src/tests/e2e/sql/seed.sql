INSERT INTO
    "user" (email)
VALUES
    ('test@email.com');

INSERT INTO "device" (name, os, user_id)
VALUES ('Pixel 8a', 'Android', (SELECT id from "user" WHERE email = 'test@email.com'));
