CREATE TABLE
    "email_confirmation" (
        "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
        "user_id" uuid NOT NULL,
        "confirmed" BOOLEAN NOT NULL DEFAULT FALSE,
        "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "confirmed_at" TIMESTAMP(3)
    );

ALTER TABLE "email_confirmation" ADD CONSTRAINT "fk_email_confirmation_user_id" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE CASCADE;

CREATE OR REPLACE FUNCTION confirmed_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.confirmed_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER table_confirmed_at_trigger
BEFORE UPDATE ON "email_confirmation"
FOR EACH ROW
EXECUTE FUNCTION confirmed_at();
