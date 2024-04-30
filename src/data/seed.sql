INSERT INTO "server" ("public_key", "backend_uri", "wireguard_uri", "country")
VALUES
    ('public_key_1', 'backend_uri_1', 'wireguard_uri_1', 'UK'),
    ('public_key_2', 'backend_uri_2', 'wireguard_uri_2', 'USA'),
    ('public_key_3', 'backend_uri_3', 'wireguard_uri_3', 'Germany');

INSERT INTO "config" ("private_key", "user_ip", "server_id")
VALUES
    ('private_key_1', '192.168.1.1', (SELECT "id" FROM "server" WHERE "public_key" = 'public_key_1')),
    ('private_key_2', '192.168.1.2', (SELECT "id" FROM "server" WHERE "public_key" = 'public_key_2')),
    ('private_key_3', '192.168.1.3', (SELECT "id" FROM "server" WHERE "public_key" = 'public_key_3'));

INSERT INTO "user" ("email", "password")
VALUES
    ('user1@example.com', '$argon2id$v=19$m=19456,t=2,p=1$zvrjYcVcpFVjHMm1CVt1OQ$ghWbtKt1bBO8uqoGnetl/V3D8Q6Cca94sHaXlZMfG0c'),
    ('user2@example.com', '$argon2id$v=19$m=19456,t=2,p=1$ebOCo1NdNvSZ0txGkhjDwQ$vQVeVGYQkccoyw/1j6OI9uZeDy8dnHeZQCiHe2TFPoY'),
    ('user3@example.com', '$argon2id$v=19$m=19456,t=2,p=1$YqgJTER84R5hCyA+7OPaUQ$xN/b5mRqYtGDv+XehAsHiJ6FQsnH+f21G86Fug5gaPk');

INSERT INTO "device" ("name", "os", "user_id")
VALUES
    ('Device1', 'Windows', (SELECT "id" FROM "user" WHERE "email" = 'user1@example.com')),
    ('Device2', 'Linux', (SELECT "id" FROM "user" WHERE "email" = 'user1@example.com')),
    ('Device3', 'MacOS', (SELECT "id" FROM "user" WHERE "email" = 'user1@example.com')),
    ('Device4', 'Android', (SELECT "id" FROM "user" WHERE "email" = 'user2@example.com')),
    ('Device5', 'IOS', (SELECT "id" FROM "user" WHERE "email" = 'user2@example.com')),
    ('Device6', 'Linux', (SELECT "id" FROM "user" WHERE "email" = 'user2@example.com')),
    ('Device7', 'MacOS', (SELECT "id" FROM "user" WHERE "email" = 'user3@example.com')),
    ('Device8', 'Windows', (SELECT "id" FROM "user" WHERE "email" = 'user3@example.com')),
    ('Device9', 'Linux', (SELECT "id" FROM "user" WHERE "email" = 'user3@example.com'));
