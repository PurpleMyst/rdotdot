#!/usr/bin/env sh

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;36m'
NC='\033[0m'

script_dir=$(readlink -f "$(dirname $0)")
output=$(realpath --relative-to=. "${script_dir}/output")

mkdir -p $output;
echo "${BLUE}[ℹ] Output directory is ${output}${NC}"

echo "${BLUE}[ℹ] Building rdotdot ...${NC}"
cargo +stable build

if [ $? -ne 0 ]; then
    echo "${RED}[✗] Something went wrong while building rdotdot${NC}"
fi

for file in $(dirname "$0")/*.ö; do
    (
        stdout_file=$(realpath --relative-to=. "${output}/$file.stdout")
        stderr_file=$(realpath --relative-to=. "${output}/$file.stderr")

        cargo +stable run "$file" > "$stdout_file" 2> "$stderr_file";

        if [ $? -eq 0 ]; then
            echo "${GREEN}[✓] Everything is fine for $file${NC}"
        else
            echo "${RED}[✗] Something went wrong for $file, you can find errors at $stderr_file${NC}"
        fi
    ) &
done

wait
