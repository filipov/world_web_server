-- Your SQL goes here
CREATE TABLE IF NOT EXISTS public.characters_with_deleted (
    id serial PRIMARY KEY NOT NULL,

    name varchar(255) NOT NULL,

    abscissa float8 NOT NULL DEFAULT 0,
    ordinate float8 NOT NULL DEFAULT 0,
    applicate float8 NOT NULL  DEFAULT 0,

    horizontal_angle int2 NOT NULL,
    vertical_angle int2 NOT NULL,

    created_at timestamp WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at timestamp WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    deleted_at timestamp WITHOUT TIME ZONE
);

SELECT diesel_manage_created_at('public.characters_with_deleted');
SELECT diesel_manage_updated_at('public.characters_with_deleted');

CREATE INDEX IF NOT EXISTS public_characters_idx ON public.characters_with_deleted(id);

CREATE INDEX IF NOT EXISTS public_characters_abscissa_idx ON public.characters_with_deleted(abscissa);
CREATE INDEX IF NOT EXISTS public_characters_ordinate_idx ON public.characters_with_deleted(ordinate);
CREATE INDEX IF NOT EXISTS public_characters_applicate_idx ON public.characters_with_deleted(applicate);

CREATE INDEX IF NOT EXISTS public_characters_coordinates_idx ON public.characters_with_deleted(abscissa, ordinate, applicate);

CREATE INDEX IF NOT EXISTS public_characters_created_at_idx ON public.characters_with_deleted(created_at);
CREATE INDEX IF NOT EXISTS public_characters_updated_at_idx ON public.characters_with_deleted(updated_at);
CREATE INDEX IF NOT EXISTS public_characters_deleted_at_idx ON public.characters_with_deleted(deleted_at);

CREATE VIEW public.characters AS SELECT * FROM public.characters_with_deleted WHERE deleted_at IS NULL;

SELECT diesel_manage_deleted_at('public.characters_with_deleted', 'public.characters');