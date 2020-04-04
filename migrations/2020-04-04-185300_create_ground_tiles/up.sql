-- Your SQL goes here
CREATE TABLE IF NOT EXISTS ground_tiles_with_deleted(
    id serial8 NOT NULL PRIMARY KEY,
    type varchar(255) NOT NULL,

    created_at timestamp WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at timestamp WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    deleted_at timestamp WITHOUT TIME ZONE
);

SELECT diesel_manage_created_at('ground_tiles_with_deleted');
SELECT diesel_manage_updated_at('ground_tiles_with_deleted');

CREATE INDEX IF NOT EXISTS ground_tiles_idx ON ground_tiles_with_deleted(id);

CREATE INDEX IF NOT EXISTS ground_tiles_type_idx ON ground_tiles_with_deleted(type);

CREATE INDEX IF NOT EXISTS ground_tiles_created_at_idx ON ground_tiles_with_deleted(created_at);
CREATE INDEX IF NOT EXISTS ground_tiles_updated_at_idx ON ground_tiles_with_deleted(updated_at);
CREATE INDEX IF NOT EXISTS ground_tiles_deleted_at_idx ON ground_tiles_with_deleted(deleted_at);

CREATE VIEW ground_tiles AS SELECT * FROM ground_tiles_with_deleted WHERE deleted_at IS NULL;


SELECT diesel_manage_deleted_at('ground_tiles_with_deleted', 'ground_tiles');