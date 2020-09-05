# Server

## Overview

## Getting Started

### Contributing

### Known Issues

## Frequently Asked Questions (F.A.Q.)

## Architecture

### Backend

Rocket

### Database

PostgreSQL

#### Schema

```
# Modify this code to update the DB schema diagram.
# To reset the sample schema, replace everything with
# two dots ('..' - without quotes).

users
-
id PK UUID
preferred_name TEXT
email TEXT
created_at TIMESTAMP
updated_at TIMESTAMP

access_types
-
value TEXT
description TEXT

access
-
id PK UUID
user_id UUID FK >- users.id
access_type TEXT FK >- access_types.value
access_digest text
created_at TIMESTAMP
updated_at TIMESTAMP

# user_details
# -
# id PK UUID
# user_id UUID FK >- users.id
# full_name string
# created_at TIMESTAMP
# updated_at TIMESTAMP
```

Use [Quick Database Diagrams](https://app.quickdatabasediagrams.com/) to see the schema visualized.

To read more about [enums in Postgres](https://hasura.io/docs/1.0/graphql/manual/schema/enums.html#enums-in-the-hasura-graphql-engine).

## Contributing

## Credits
