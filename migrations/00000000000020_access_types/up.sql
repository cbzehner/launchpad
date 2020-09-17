CREATE TABLE IF NOT EXISTS access_types (
  value text PRIMARY KEY,
  description text
);

COMMENT ON TABLE access_types IS 'Enumeration of the possible 3rd party applications protocols.';

INSERT INTO access_types (value, description) VALUES
  ('password', 'Standard authentication method using an email and password pair'),
  ('google_oauth2', 'Authenticate using Google as an OAuth 2.0 identity provider'),
  ('github_oauth2', 'Authenticate using GitHub as an OAuth 2.0 identity provider'),
  ('microsoft_oauth2', 'Authenticate using Microsoft as an OAuth 2.0 identity provider');
