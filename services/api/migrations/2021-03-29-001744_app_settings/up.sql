CREATE TABLE IF NOT EXISTS app_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID UNIQUE NOT NULL, -- Only one row is allowed per user account
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp()
);

COMMENT ON TABLE app_settings IS 'User settings specific to the application.';
COMMENT ON COLUMN app_settings.user_id IS 'The user to whom these settings relate.';

-- Use the trigger to manage the `updated_at` value for this table.
SELECT diesel_manage_updated_at('app_settings');