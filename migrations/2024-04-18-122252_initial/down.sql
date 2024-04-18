ALTER TABLE "Session"
DROP CONSTRAINT IF EXISTS "Session_deviceId_fkey";

ALTER TABLE "Session"
DROP CONSTRAINT IF EXISTS "Session_configId_fkey";

ALTER TABLE "Device"
DROP CONSTRAINT IF EXISTS "Device_userId_fkey";

ALTER TABLE "Config"
DROP CONSTRAINT IF EXISTS "Config_serverId_fkey";

DROP INDEX IF EXISTS "Session_configId_key";

DROP INDEX IF EXISTS "Session_deviceId_key";

DROP INDEX IF EXISTS "User_email_key";

DROP INDEX IF EXISTS "Device_pkey";

DROP INDEX IF EXISTS "Config_privateKey_key";

DROP INDEX IF EXISTS "Server_wireguardUri_key";

DROP INDEX IF EXISTS "Server_backendUri_key";

DROP INDEX IF EXISTS "Server_publicKey_key";

DROP TABLE IF EXISTS "Session";

DROP TABLE IF EXISTS "Device";

DROP TABLE IF EXISTS "User";

DROP TABLE IF EXISTS "Config";

DROP TABLE IF EXISTS "Server";

DROP TYPE IF EXISTS "DeviceStatus";

DROP TYPE IF EXISTS "OS";

DROP TYPE IF EXISTS "UserStatus";

DROP TYPE IF EXISTS "ConfigStatus";

DROP TYPE IF EXISTS "SessionStatus";

DROP TYPE IF EXISTS "Country";