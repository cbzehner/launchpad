CREATE TABLE IF NOT EXISTS user_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    preferred_name TEXT NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT 'false',
    last_login_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp()
);

COMMENT ON TABLE user_settings IS 'Settings related to a user account.';
COMMENT ON COLUMN user_settings.preferred_name IS 'Display name of the user.';
COMMENT ON COLUMN user_settings.verified IS 'Whether the user has proved they own the email address they used to sign up.';
COMMENT ON COLUMN user_settings.last_login_at IS 'The timestamp of the last time the user logged into the application.';
