-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();

DROP FUNCTION IF EXISTS diesel_manage_created_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_created_at();
DROP FUNCTION IF EXISTS diesel_safe_created_at();

DROP FUNCTION IF EXISTS diesel_manage_deleted_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_deleted_at();
DROP FUNCTION IF EXISTS diesel_safe_deleted_at();
DROP FUNCTION IF EXISTS diesel_soft_delete();
