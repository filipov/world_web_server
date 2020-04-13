-- Your SQL goes here
CREATE TABLE IF NOT EXISTS ground_tiles_vertexes_with_deleted(
    tile_id int8 NOT NULL,
    vertex_id int8 NOT NULL,

    CONSTRAINT PK PRIMARY KEY (tile_id, vertex_id),

    FOREIGN KEY (tile_id) REFERENCES ground_tiles_with_deleted(id) ON DELETE RESTRICT,
    FOREIGN KEY (vertex_id) REFERENCES ground_vertexes_with_deleted(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS ground_tiles_vertexes_tile_idx ON ground_tiles_vertexes_with_deleted(tile_id);
CREATE INDEX IF NOT EXISTS ground_tiles_vertexes_vertex_idx ON ground_tiles_vertexes_with_deleted(vertex_id);

CREATE UNIQUE INDEX IF NOT EXISTS ground_tiles_vertexes_idx ON ground_tiles_vertexes_with_deleted(tile_id, vertex_id);

CREATE VIEW ground_tiles_vertexes AS
    SELECT ground_tiles_vertexes_with_deleted.* FROM ground_tiles_vertexes_with_deleted
      INNER JOIN ground_tiles ON ground_tiles_vertexes_with_deleted.tile_id = ground_tiles.id
      INNER JOIN ground_vertexes ON ground_tiles_vertexes_with_deleted.vertex_id = ground_vertexes.id;
