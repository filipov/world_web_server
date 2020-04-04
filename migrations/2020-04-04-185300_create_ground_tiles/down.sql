-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS ground_tiles_idx;

DROP INDEX IF EXISTS ground_tiles_type_idx;

DROP INDEX IF EXISTS ground_tiles_created_at_idx;
DROP INDEX IF EXISTS ground_tiles_updated_at_idx;
DROP INDEX IF EXISTS ground_tiles_deleted_at_idx;

DROP VIEW IF EXISTS ground_tiles;

DROP TABLE IF EXISTS ground_tiles_with_deleted;