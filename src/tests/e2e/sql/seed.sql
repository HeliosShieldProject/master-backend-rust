INSERT INTO "server" ("public_key", "wireguard_uri", "country")
VALUES
    ('public_key_1', 'wireguard_uri_1', 'UK'),
    ('public_key_2', 'wireguard_uri_2', 'USA'),
    ('public_key_3', 'wireguard_uri_3', 'Germany');

INSERT INTO "config" ("private_key", "user_ip", "server_id")
VALUES
    ('private_key_1', '192.168.1.1', (SELECT "id" FROM "server" WHERE "public_key" = 'public_key_1')),
    ('private_key_2', '192.168.1.2', (SELECT "id" FROM "server" WHERE "public_key" = 'public_key_2')),
    ('private_key_3', '192.168.1.3', (SELECT "id" FROM "server" WHERE "public_key" = 'public_key_3'));
