-- Drop triggers
DROP TRIGGER table_updated_at_trigger ON "server";

DROP TRIGGER table_updated_at_trigger ON "config";

DROP TRIGGER table_updated_at_trigger ON "user";

DROP TRIGGER table_updated_at_trigger ON "device";

-- Drop function
DROP FUNCTION update_updated_at ();

-- Drop foreign key constraints
ALTER TABLE "session"
DROP CONSTRAINT "session_config_id_fkey";

ALTER TABLE "session"
DROP CONSTRAINT "session_device_id_fkey";

ALTER TABLE "device"
DROP CONSTRAINT "device_user_id_fkey";

ALTER TABLE "config"
DROP CONSTRAINT "config_server_id_fkey";

-- Drop indexes
DROP INDEX "session_configId_key";

DROP INDEX "session_deviceId_key";

DROP INDEX "user_email_key";

DROP INDEX "config_privateKey_key";

DROP INDEX "server_wireguardUri_key";

DROP INDEX "server_backendUri_key";

DROP INDEX "server_publicKey_key";

-- Drop tables
DROP TABLE "session";

DROP TABLE "device";

DROP TABLE "user";

DROP TABLE "config";

DROP TABLE "server";

-- Drop types
DROP TYPE "UserStatus";

DROP TYPE "DeviceStatus";

DROP TYPE "OS";

DROP TYPE "ConfigStatus";

DROP TYPE "SessionStatus";

DROP TYPE "Country";

-- Drop extension
DROP EXTENSION "uuid-ossp";