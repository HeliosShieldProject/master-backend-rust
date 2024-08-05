CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE "Country" AS ENUM ('UK', 'USA', 'Germany');

CREATE TYPE "SessionStatus" AS ENUM ('Active', 'Closed');

CREATE TYPE "ConfigStatus" AS ENUM ('InUse', 'NotInUse');

CREATE TYPE "OS" AS ENUM (
    'Windows',
    'Linux',
    'MacOS',
    'IOS',
    'Android',
    'Unknown'
);

CREATE TYPE "DeviceStatus" AS ENUM ('LoggedIn', 'LoggedOut');

CREATE TYPE "UserStatus" AS ENUM (
    'Active',
    'Banned',
    'PermanentlyBanned',
    'Deleted'
);

CREATE TABLE
    "server" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "public_key" TEXT NOT NULL,
        "wireguard_uri" TEXT NOT NULL,
        "country" "Country" NOT NULL,
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "config" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "private_key" TEXT NOT NULL,
        "user_ip" TEXT NOT NULL,
        "server_id" uuid NOT NULL,
        "status" "ConfigStatus" NOT NULL DEFAULT 'NotInUse',
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "user" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "email" TEXT NOT NULL,
        "password" TEXT NOT NULL,
        "banned_at" TIMESTAMP(3),
        "banned_till" TIMESTAMP(3),
        "status" "UserStatus" NOT NULL DEFAULT 'Active',
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "device" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "name" TEXT NOT NULL,
        "os" "OS" NOT NULL,
        "user_id" uuid NOT NULL,
        "banned_at" TIMESTAMP(3),
        "banned_till" TIMESTAMP(3),
        "revoked_at" TIMESTAMP(3),
        "status" "DeviceStatus" NOT NULL DEFAULT 'LoggedIn',
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "session" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "status" "SessionStatus" NOT NULL DEFAULT 'Active',
        "opened_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "closed_at" TIMESTAMP(3),
        "device_id" uuid NOT NULL,
        "config_id" uuid NOT NULL
    );

CREATE UNIQUE INDEX "server_publicKey_key" ON "server" ("public_key");

CREATE UNIQUE INDEX "server_wireguardUri_key" ON "server" ("wireguard_uri");

CREATE INDEX "config_privateKey_key" ON "config" ("private_key");

CREATE UNIQUE INDEX "user_email_key" ON "user" ("email");

CREATE INDEX "session_deviceId_key" ON "session" ("device_id");

CREATE INDEX "session_configId_key" ON "session" ("config_id");

ALTER TABLE "config" ADD CONSTRAINT "config_server_id_fkey" FOREIGN KEY ("server_id") REFERENCES "server" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "device" ADD CONSTRAINT "device_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "session" ADD CONSTRAINT "session_device_id_fkey" FOREIGN KEY ("device_id") REFERENCES "device" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "session" ADD CONSTRAINT "session_config_id_fkey" FOREIGN KEY ("config_id") REFERENCES "config" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER table_updated_at_trigger
BEFORE UPDATE ON "server"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();


CREATE TRIGGER table_updated_at_trigger
BEFORE UPDATE ON "config"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();


CREATE TRIGGER table_updated_at_trigger
BEFORE UPDATE ON "user"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();


CREATE TRIGGER table_updated_at_trigger
BEFORE UPDATE ON "device"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();