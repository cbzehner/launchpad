-- SQL commands to run when initializing a new database instance
-- Create a database for each service that interacts directly with Postgres
-- and assign each database a unique role to limit their data access.

-- TODO: Should there be a separate LOGIN role? See https://www.postgresql.org/docs/current/sql-createrole.html

CREATE ROLE kratos WITH LOGIN PASSWORD 'secret'; -- TODO: Change this to be secure!
CREATE DATABASE kratos;
GRANT ALL PRIVILEGES ON DATABASE kratos TO kratos;

CREATE ROLE api WITH LOGIN PASSWORD 'secret'; -- TODO: Change this to be secure!
CREATE DATABASE api;
GRANT ALL PRIVILEGES ON DATABASE api TO api;
CREATE DATABASE api_test;
GRANT ALL PRIVILEGES ON DATABASE api_test TO api;