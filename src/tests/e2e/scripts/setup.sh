original_dir=$(pwd)
function return_to_original_dir {
    cd "$original_dir"
}
trap return_to_original_dir EXIT
cd "$(git rev-parse --show-toplevel)"

docker compose -f docker-compose-test.yml down

docker compose --env-file .env.test -f docker-compose-test.yml up -d
while ! pg_isready -h localhost -p 5555 -U helios; do
    sleep 1
done

find migrations -type f -name "up.sql" | while read file; do
    psql postgresql://helios:123456789@localhost:5555/heliosdb -f $file
done
psql postgresql://helios:123456789@localhost:5555/heliosdb -f src/tests/e2e/sql/seed.sql

cd "$original_dir"
