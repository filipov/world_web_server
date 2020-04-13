-- Your SQL goes here
CREATE VIEW grounds AS
    SELECT
           grnds.id,
           grnds.types[1],
           array_to_json(grnds.vertexes) as vertexes,
           array_to_json(grnds.angles) as angles,
           array_to_json(grnds.vectors) as vectors FROM
(
    SELECT
        ground_tiles_vertexes.tile_id as id,
        array_agg(ground_tiles.type) as types,
        array_agg(
            array[
                ground_vertexes.abscissa,
                ground_vertexes.ordinate,
                ground_vertexes.applicate
            ]
        ) as vertexes,
        array_agg(
            array[
                ground_vertexes.horizontal_angle,
                ground_vertexes.vertical_angle
            ]
        ) as angles,
        array_agg(
            array[
                ground_vertexes.abscissa,
                ground_vertexes.ordinate,
                ground_vertexes.applicate,
                ground_vertexes.horizontal_angle,
                ground_vertexes.vertical_angle
            ]
        ) as vectors

    FROM ground_tiles_vertexes

        INNER JOIN ground_tiles
            ON ground_tiles.id = ground_tiles_vertexes.tile_id

        INNER JOIN ground_vertexes
            ON ground_vertexes.id = ground_tiles_vertexes.vertex_id

    GROUP BY ground_tiles_vertexes.tile_id
) as grnds;