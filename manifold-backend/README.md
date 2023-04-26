# Manifold Backend

This is the backend for Manifold.

---

## Style Guide

### Routes

Routes should use `kebab-case` formatting, and should not contain any uppercase characters.

---

## Database

### ID

IDs should be stored as type `binary(16) NOT NULL DEFAULT (uuid_to_bin(uuid(), true)) PRIMARY KEY`

- `binary(16)`: stored as 16 unformatted bytes of data. The other option is to use the type `chars(36)` but that uses
  UTF8 character encoding, which has a variable byte length for each character, so at minimum the amount of bytes used
  would be 36 but it could go up to 144 bytes. There may be slightly more overhead with having to transfer UUID string
  into `binary(16)` and back, but there is a much smaller storage size and indexing speeds are greatly increased.
- `NOT NULL`: There should never be any entry with an id where the id value is null, especially as the id is used as a
  primary key.
- `DEFAULT (uuid_to_bin(uuid(), true))`: Sets so that the `id` field is not required to be set when inserting into the
  table, as this will automatically generate a correctly encoded id value automatically. You can still provide an id
  value when inserting though, if that is preferred, you just need to remember to manually call `uuid_to_bin` on the
  UUID string value, ensuring to pass `true` as the second parameter of the function. The `uuid_to_bin` function
  converts a UUID string into its binary representation, and there is a related `bin_to_uuid` funcion which can be used
  to convert the binary representation back into a UUID string. The `uuid` function generates a UUID string. The second
  value in the `uuid_to_bin` function changes the position of the temporal bits in the UUID, which provides better
  indexing performance in the database. Ensure that you always pass `true` as the second parameter to either the
  `uuid_to_bin` or `bin_to_uuid` function.
- `PRIMARY KEY`: This sets the id as the primary key value of the database. This ensures that each entry in the column
  is always unique.
