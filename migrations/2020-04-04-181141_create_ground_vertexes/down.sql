-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS ground_vertexes_idx;

DROP INDEX IF EXISTS ground_vertexes_abscissa_idx;
DROP INDEX IF EXISTS ground_vertexes_ordinate_idx;
DROP INDEX IF EXISTS ground_vertexes_applicate_idx;

DROP INDEX IF EXISTS ground_vertexes_coordinates_idx;

DROP INDEX IF EXISTS ground_vertexes_horizontal_angle_idx;
DROP INDEX IF EXISTS ground_vertexes_vertical_angle_idx;

DROP INDEX IF EXISTS ground_vertexes_type_idx;

DROP INDEX IF EXISTS ground_vertexes_coordinates_type_idx;

DROP INDEX IF EXISTS ground_vertexes_created_at_idx;
DROP INDEX IF EXISTS ground_vertexes_updated_at_idx;
DROP INDEX IF EXISTS ground_vertexes_deleted_at_idx;

DROP VIEW IF EXISTS ground_vertexes;

DROP TABLE IF EXISTS ground_vertexes_with_deleted;