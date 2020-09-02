CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    preferred_name TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT transaction_timestamp()
)
