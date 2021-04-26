# Docker

### Ports
A mapping of ports used internally when running `docker-compose`. Only ports `80` (http) and `443` (https) are exposed publically.

Port   | Service        | Description
------ | ------         |----------
`4433` | ORY Kratos     | Administrative API endpoints
`4433` | ORY Kratos     | Public API endpoints
`4435` | React Web App  | Access the frontend container
`4436` | Mailslurper    | Local web-based mail server
`4437` | Mailslurper    | Local web-based mail server
`4438` | Backend API    | The primary API endpoint
`4455` | ORY Oathkeeper | Public endpoint for accessing
`4456` | ORY Oathkeeper | $
`5432` | Postgres       | SQL server instance
