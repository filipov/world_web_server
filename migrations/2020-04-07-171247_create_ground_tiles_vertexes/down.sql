-- This file should undo anything in `up.sql`
DROP VIEW IF EXISTS ground_tiles_vertexes;

DROP INDEX IF EXISTS ground_tiles_vertexes_tile_idx;
DROP INDEX IF EXISTS ground_tiles_vertexes_vertex_idx;

DROP INDEX IF EXISTS ground_tiles_vertexes_idx;

DROP TABLE IF EXISTS ground_tiles_vertexes_with_deleted;