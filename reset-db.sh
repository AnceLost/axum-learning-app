#!/usr/bin/env bash
set -euo pipefail

# пишите свои
CONTAINER="db"
DB_USER="appuser"
DB_NAME="appdb"

docker compose exec -T "$CONTAINER" psql -U "$DB_USER" -d "$DB_NAME" -c "
DO \$\$
DECLARE
    r RECORD;
BEGIN
    FOR r IN
        SELECT tablename FROM pg_tables
        WHERE schemaname = 'public'
          AND tablename NOT IN ('seaql_migrations')   -- ← ключевое отличие
    LOOP
        EXECUTE 'TRUNCATE TABLE ' || quote_ident(r.tablename) || ' RESTART IDENTITY CASCADE;';
    END LOOP;
END \$\$;
"