original_dir=$(pwd)
function return_to_original_dir {
    cd "$original_dir"
}
trap return_to_original_dir EXIT
cd "$(git rev-parse --show-toplevel)"

docker compose --env-file .env.test -f docker-compose-test.yml up -d
while ! pg_isready -h localhost -p 5555 -U helios; do
    sleep 1
done

psql postgresql://helios:123456789@localhost:5555/heliosdb -f migrations/00000000000000_diesel_initial_setup/up.sql
psql postgresql://helios:123456789@localhost:5555/heliosdb -f migrations/2024-08-02-114025_initial/up.sql
psql postgresql://helios:123456789@localhost:5555/heliosdb -f src/tests/e2e/sql/seed.sql

cargo test e2e

docker compose -f docker-compose-test.yml down

cd "$original_dir"
