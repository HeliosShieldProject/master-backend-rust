CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE "Country" AS ENUM('UK', 'USA', 'Germany');

CREATE TYPE "DeviceStatus" AS ENUM('LoggedIn', 'LoggedOut');

CREATE TYPE "OS" AS ENUM('Windows', 'Linux', 'MacOS', 'IOS', 'Android', 'Unknown');

CREATE TYPE "SessionStatus" AS ENUM('Active', 'Closed');

CREATE TYPE "UserStatus" AS ENUM('Active', 'Banned', 'PermanentlyBanned', 'Deleted');

CREATE TYPE "OAuthProvider" AS ENUM('Google', 'Discord', 'GitHub');

CREATE TYPE "Protocol" AS ENUM('Vless', 'Shadowsocks');

CREATE TABLE
    "user" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "email" TEXT NOT NULL,
        "status" "UserStatus" NOT NULL DEFAULT 'Active',
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "classic_auth" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "user_id" uuid NOT NULL,
        "password_hash" TEXT NOT NULL,
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "oauth" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "user_id" uuid NOT NULL,
        "provider" TEXT NOT NULL,
        "metadata" JSONB NOT NULL,
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "email_confirmation" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "user_id" uuid NOT NULL,
        "confirmed" BOOLEAN NOT NULL DEFAULT FALSE,
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "confirmed_at" TIMESTAMP(3)
    );

CREATE TABLE
    "device" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "name" TEXT NOT NULL,
        "os" "OS" NOT NULL,
        "user_id" uuid NOT NULL,
        "status" "DeviceStatus" NOT NULL DEFAULT 'LoggedIn',
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "session" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "device_id" uuid NOT NULL,
        "link" TEXT NOT NULL,
        "protocol" "Protocol" NOT NULL,
        "country" "Country" NOT NULL,
        "status" "SessionStatus" NOT NULL DEFAULT 'Active',
        "up" BIGINT,
        "down" BIGINT,
        "opened_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "closed_at" TIMESTAMP(3)
    );

CREATE UNIQUE INDEX "user_email_key" ON "user" ("email");

CREATE INDEX "session_device_id_key" ON "session" ("device_id");

CREATE UNIQUE INDEX "session_device_id_active_status_key" ON "session" ("device_id")
WHERE
    status = 'Active';

ALTER TABLE "device"
ADD CONSTRAINT "device_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "session"
ADD CONSTRAINT "session_device_id_fkey" FOREIGN KEY ("device_id") REFERENCES "device" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "classic_auth"
ADD CONSTRAINT "classic_auth_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "oauth"
ADD CONSTRAINT "oauth_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "email_confirmation"
ADD CONSTRAINT "fk_email_confirmation_user_id" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE CASCADE;

CREATE
OR REPLACE FUNCTION confirmed_at () RETURNS TRIGGER AS $$
BEGIN
    NEW.confirmed_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER table_confirmed_at_trigger BEFORE
UPDATE ON "email_confirmation" FOR EACH ROW
EXECUTE FUNCTION confirmed_at ();

CREATE
OR REPLACE FUNCTION update_updated_at () RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER table_updated_at_trigger BEFORE
UPDATE ON "user" FOR EACH ROW
EXECUTE FUNCTION update_updated_at ();

CREATE TRIGGER table_updated_at_trigger BEFORE
UPDATE ON "device" FOR EACH ROW
EXECUTE FUNCTION update_updated_at ();

CREATE TRIGGER table_updated_at_trigger BEFORE
UPDATE ON "classic_auth" FOR EACH ROW
EXECUTE FUNCTION update_updated_at ();

CREATE TRIGGER table_updated_at_trigger BEFORE
UPDATE ON "oauth" FOR EACH ROW
EXECUTE FUNCTION update_updated_at ();