DROP TRIGGER IF EXISTS table_updated_at_trigger ON "user";

DROP TRIGGER IF EXISTS table_updated_at_trigger ON "device";

DROP TRIGGER IF EXISTS table_updated_at_trigger ON "classic_auth";

DROP TRIGGER IF EXISTS table_updated_at_trigger ON "oauth";

DROP TRIGGER IF EXISTS table_confirmed_at_trigger ON "email_confirmation";

DROP FUNCTION IF EXISTS update_updated_at ();

DROP FUNCTION IF EXISTS confirmed_at ();

ALTER TABLE "device"
DROP CONSTRAINT IF EXISTS "device_user_id_fkey";

ALTER TABLE "session"
DROP CONSTRAINT IF EXISTS "session_device_id_fkey";

ALTER TABLE "classic_auth"
DROP CONSTRAINT IF EXISTS "classic_auth_user_id_fkey";

ALTER TABLE "oauth"
DROP CONSTRAINT IF EXISTS "oauth_user_id_fkey";

ALTER TABLE "email_confirmation"
DROP CONSTRAINT IF EXISTS "fk_email_confirmation_user_id";

DROP INDEX IF EXISTS "user_email_key";

DROP INDEX IF EXISTS "session_device_id_key";

DROP INDEX IF EXISTS "session_device_id_active_status_key";

DROP TABLE IF EXISTS "session";

DROP TABLE IF EXISTS "device";

DROP TABLE IF EXISTS "email_confirmation";

DROP TABLE IF EXISTS "oauth";

DROP TABLE IF EXISTS "classic_auth";

DROP TABLE IF EXISTS "user";

DROP TYPE IF EXISTS "Protocol";

DROP TYPE IF EXISTS "OAuthProvider";

DROP TYPE IF EXISTS "UserStatus";

DROP TYPE IF EXISTS "SessionStatus";

DROP TYPE IF EXISTS "OS";

DROP TYPE IF EXISTS "DeviceStatus";

DROP TYPE IF EXISTS "Country";

DROP EXTENSION IF EXISTS "uuid-ossp";