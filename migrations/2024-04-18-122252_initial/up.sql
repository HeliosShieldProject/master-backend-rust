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

CREATE TYPE "DeviceStatus" AS ENUM ('LoggedIn', 'LoggedOut', 'Revoked');

CREATE TYPE "UserStatus" AS ENUM (
    'Active',
    'Banned',
    'PermanentlyBanned',
    'Deleted'
);

CREATE TABLE
    "Server" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "public_key" TEXT NOT NULL,
        "backend_uri" TEXT NOT NULL,
        "wireguard_uri" TEXT NOT NULL,
        "country" "Country" NOT NULL,
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "Config" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "private_key" TEXT NOT NULL,
        "user_ip" TEXT NOT NULL,
        "server_id" uuid NOT NULL,
        "status" "ConfigStatus" NOT NULL DEFAULT 'NotInUse',
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "User" (
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
    "Device" (
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
    "Session" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "status" "SessionStatus" NOT NULL DEFAULT 'Active',
        "opened_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "closed_at" TIMESTAMP(3),
        "device_id" uuid NOT NULL,
        "config_id" uuid NOT NULL
    );

CREATE UNIQUE INDEX "Server_publicKey_key" ON "Server" ("public_key");

CREATE UNIQUE INDEX "Server_backendUri_key" ON "Server" ("backend_uri");

CREATE UNIQUE INDEX "Server_wireguardUri_key" ON "Server" ("wireguard_uri");

CREATE INDEX "Config_privateKey_key" ON "Config" ("private_key");

CREATE UNIQUE INDEX "User_email_key" ON "User" ("email");

CREATE INDEX "Session_deviceId_key" ON "Session" ("device_id");

CREATE INDEX "Session_configId_key" ON "Session" ("config_id");

ALTER TABLE "Config" ADD CONSTRAINT "Config_server_id_fkey" FOREIGN KEY ("server_id") REFERENCES "Server" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "Device" ADD CONSTRAINT "Device_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "User" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "Session" ADD CONSTRAINT "Session_device_id_fkey" FOREIGN KEY ("device_id") REFERENCES "Device" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "Session" ADD CONSTRAINT "Session_config_id_fkey" FOREIGN KEY ("config_id") REFERENCES "Config" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER table_updated_at_trigger
BEFORE UPDATE ON "Server"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();


CREATE TRIGGER table_updated_at_trigger
BEFORE UPDATE ON "Config"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();


CREATE TRIGGER table_updated_at_trigger
BEFORE UPDATE ON "User"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();


CREATE TRIGGER table_updated_at_trigger
BEFORE UPDATE ON "Device"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();