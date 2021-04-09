CREATE TYPE pg_json_validate_draft AS enum ('Draft4', 'Draft6', 'Draft7');
CREATE TABLE pg_json_validate
(
  id text NOT NULL PRIMARY KEY,
  schema jsonb NOT NULL,
  draft pg_json_validate_draft NOT NULL
);