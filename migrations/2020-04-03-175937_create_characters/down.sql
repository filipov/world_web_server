-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS public_characters_idx;

DROP INDEX IF EXISTS public_characters_abscissa_idx;
DROP INDEX IF EXISTS public_characters_ordinate_idx;
DROP INDEX IF EXISTS public_characters_applicate_idx;

DROP INDEX IF EXISTS public_characters_coordinates_idx;

DROP INDEX IF EXISTS public_characters_created_at_idx;
DROP INDEX IF EXISTS public_characters_updated_at_idx;
DROP INDEX IF EXISTS public_characters_deleted_at_idx;

DROP VIEW IF EXISTS public.characters;

DROP TABLE IF EXISTS public.characters_with_deleted;