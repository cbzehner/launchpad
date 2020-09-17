-- TODO (correctness): Investigate different 3rd party APIs and redesign this table.
--                     This effort is currently incomplete and the design is unproven.
CREATE TABLE IF NOT EXISTS user_access_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    access_type TEXT REFERENCES access_types(value) ON DELETE RESTRICT NOT NULL,
    token TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp(),
    UNIQUE (user_id, access_type)
);

COMMENT ON TABLE user_access_tokens IS 'Store user credentials for 3rd party applications.';
COMMENT ON COLUMN user_access_tokens.access_type IS 'An enumeration of the possible 3rd party applications.';
COMMENT ON COLUMN user_access_tokens.token IS 'The token or secret used for accessing a 3rd party application.';

