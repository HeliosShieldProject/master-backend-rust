DROP TRIGGER table_confirmed_at_trigger ON "email_confirmation";

DROP FUNCTION confirmed_at ();

DROP
ALTER TABLE "email_confirmation"
DROP CONSTRAINT "fk_email_confirmation_user_id";

DROP TABLE "email_confirmation";