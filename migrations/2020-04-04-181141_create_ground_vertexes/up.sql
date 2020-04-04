-- Your SQL goes here
CREATE TABLE IF NOT EXISTS ground_vertexes_with_deleted (
    id serial8 NOT NULL PRIMARY KEY,
    abscissa float8 NOT NULL DEFAULT 0.0,
    ordinate float8 NOT NULL DEFAULT 0.0,
    applicate float8 NOT NULL DEFAULT 0.0,

    horizontal_angle float NOT NULL DEFAULT 0.0,
    vertical_angle float NOT NULL DEFAULT 0.0,

    type varchar(255) NOT NULL,

    created_at timestamp WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at timestamp WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    deleted_at timestamp WITHOUT TIME ZONE
);

SELECT diesel_manage_created_at('ground_vertexes_with_deleted');
SELECT diesel_manage_updated_at('ground_vertexes_with_deleted');

CREATE INDEX IF NOT EXISTS ground_vertexes_idx ON ground_vertexes_with_deleted(id);

CREATE INDEX IF NOT EXISTS ground_vertexes_abscissa_idx ON ground_vertexes_with_deleted(abscissa);
CREATE INDEX IF NOT EXISTS ground_vertexes_ordinate_idx ON ground_vertexes_with_deleted(ordinate);
CREATE INDEX IF NOT EXISTS ground_vertexes_applicate_idx ON ground_vertexes_with_deleted(applicate);

CREATE UNIQUE INDEX IF NOT EXISTS ground_vertexes_coordinates_idx
    ON ground_vertexes_with_deleted(abscissa, ordinate, applicate)
    WHERE deleted_at IS NOT NULL;

CREATE INDEX IF NOT EXISTS ground_vertexes_horizontal_angle_idx ON ground_vertexes_with_deleted(horizontal_angle);
CREATE INDEX IF NOT EXISTS ground_vertexes_vertical_angle_idx ON ground_vertexes_with_deleted(vertical_angle);

CREATE INDEX IF NOT EXISTS ground_vertexes_type_idx ON ground_vertexes_with_deleted(type);

CREATE UNIQUE INDEX IF NOT EXISTS ground_vertexes_coordinates_type_idx
    ON ground_vertexes_with_deleted(abscissa, ordinate, applicate, type)
    WHERE deleted_at IS NOT NULL;

CREATE INDEX IF NOT EXISTS ground_vertexes_created_at_idx ON ground_vertexes_with_deleted(created_at);
CREATE INDEX IF NOT EXISTS ground_vertexes_updated_at_idx ON ground_vertexes_with_deleted(updated_at);
CREATE INDEX IF NOT EXISTS ground_vertexes_deleted_at_idx ON ground_vertexes_with_deleted(deleted_at);

CREATE VIEW ground_vertexes AS SELECT * FROM ground_vertexes_with_deleted WHERE deleted_at IS NULL;


SELECT diesel_manage_deleted_at('ground_vertexes_with_deleted', 'ground_vertexes');