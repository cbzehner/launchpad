CREATE TABLE IF NOT EXISTS access (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE RESTRICT NOT NULL,
    access_type TEXT REFERENCES access_types(value) ON DELETE RESTRICT NOT NULL,
    access_digest TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp(),
    UNIQUE (user_id, access_type)
)
