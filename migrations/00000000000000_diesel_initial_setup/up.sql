-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    NEW.updated_at := current_timestamp;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, created_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_created_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_created_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_created_at BEFORE INSERT ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_created_at()', _tbl);
    EXECUTE format('CREATE TRIGGER safe_created_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_safe_created_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_created_at() RETURNS trigger AS $$
BEGIN
    NEW.created_at := current_timestamp;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_safe_created_at() RETURNS trigger AS $$
BEGIN
    NEW.created_at := OLD.created_at;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, deleted_at TIMESTAMP);
--
-- SELECT diesel_manage_deleted_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_deleted_at(_tbl regclass, _view regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_deleted_at BEFORE INSERT ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_deleted_at()', _tbl);
    EXECUTE format('CREATE TRIGGER safe_deleted_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_safe_created_at()', _tbl);
    EXECUTE format('CREATE TRIGGER soft_delete BEFORE DELETE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_soft_delete()', _tbl);
    EXECUTE format('CREATE TRIGGER soft_delete INSTEAD OF DELETE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_soft_delete()', _view);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_deleted_at() RETURNS trigger AS $$
BEGIN
    NEW.deleted_at := NULL;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_safe_deleted_at() RETURNS trigger AS $$
BEGIN
    IF NEW.deleted_at IS NOT NULL
        THEN
            NEW.deleted_at := OLD.deleted_at;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_soft_delete() RETURNS trigger AS $$
BEGIN
    EXECUTE format('UPDATE %s SET deleted_at = current_timestamp WHERE id = %s', TG_TABLE_NAME, OLD.id);

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
