original_dir=$(pwd)
function return_to_original_dir {
    cd "$original_dir"
}
trap return_to_original_dir EXIT
cd "$(git rev-parse --show-toplevel)"

psql postgresql://helios:123456789@localhost:5555/heliosdb -f src/tests/e2e/sql/dump.sql
psql postgresql://helios:123456789@localhost:5555/heliosdb -f src/tests/e2e/sql/seed.sql

cd "$original_dir"
