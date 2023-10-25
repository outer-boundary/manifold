# Manifold Backend

This is the backend for Manifold.

---

## Setup

### .env file

Create a `.env` file in the root of the project and populate it using the template below:

```
COCKROACH_PASSWORD=...
MANIFOLD__DATABASE__URL="postgresql://root:${COCKROACH_PASSWORD}@127.0.0.1:26257/core?sslmode=disable"
# This is required for the compile-time checking done by sqlx
DATABASE_URL=${MANIFOLD__DATABASE__URL}

MANIFOLD__AUTHENTICATION__PEPPER=...
```

The `...` represent values that you need to fill in yourself.

### pnpm

To install pnpm, first install the latest LTS version of Node, and then run the following commands:

`corepack enable`

`corepack prepare pnpm@latest --activate`

This will enable you to use pnpm as your node package manager.

### Argonautica

`argonautica` is the cryptography package we use for password hashing. To be able to use the crate, a C compiler is needed
to compile the C implementation of Argon2. Follow the instructions
[here](https://docs.rs/argonautica/latest/argonautica/#installation) to set up your environment to be able to use
`argonautica`.

Ensure that you restart your system after installation to ensure that the `argonautica` crate can find the installed C
compiler.

### Docker Desktop

Follow [these instructions](https://docs.docker.com/desktop/install/windows-install/) to get set up with docker on Windows.

### DBeaver

Follow [these instructions](https://www.cockroachlabs.com/docs/v23.1/dbeaver) to install DBeaver and connect it to CockroachDB in docker, which will allow you to manage your database locally.

---

## Database

### Migrations

When developping locally, run `pnpm run migrations:add {migration_name}` to create a new migration file inside the
`migrations` folder. This file can be used to apply schema changes to the database. Once you have finished making your
schema changes, run `pnpm run migrations:run` to apply all migrations to your local database. This will also be done
automatically every time the backend is started.

---

## Style/Formatting Guide

### Routes

Routes should use `kebab-case` formatting, and should not contain any uppercase characters.
