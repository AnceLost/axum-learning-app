#!/usr/bin/env bash
set -euo pipefail

# Настройки — подставьте свои
CONTAINER="db"
DB_USER="appuser"
DB_NAME="appdb"

echo "Resetting database '${DB_NAME}' in container '${CONTAINER}'..."

docker compose exec -T "$CONTAINER" psql -U "$DB_USER" -d "$DB_NAME" -c "
DO \$\$
DECLARE
    r RECORD;
BEGIN
    FOR r IN SELECT tablename FROM pg_tables WHERE schemaname = 'public'
    LOOP
        EXECUTE 'TRUNCATE TABLE ' || quote_ident(r.tablename) || ' RESTART IDENTITY CASCADE;';
    END LOOP;
END \$\$;
"

echo "Done. All tables truncated and identity sequences restarted."