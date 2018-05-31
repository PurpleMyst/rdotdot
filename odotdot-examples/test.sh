#!/usr/bin/env sh

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;36m'
NC='\033[0m'

info() {
    echo "${BLUE}[ℹ]" "$@" "${NC}"
}

success() {
    echo "${GREEN}[✓]" "$@" "${NC}"
}

failure() {
    echo "${RED}[✗]" "$@" "${NC}"
}

script_dir=$(readlink -f "$(dirname "$0")")
output=$(realpath --relative-to=. "${script_dir}/output")

mkdir -p "$output"
info "Output directory is ${output}" 1

info "Testing rdotdot ..."
cargo +stable test

if [ $? -eq 0 ]; then
    success "Everything is fine for rdotdot"
else
    failure "Something went wrong while testing rdotdot, exiting"
    exit 1
fi

for file in $(dirname "$0")/*.ö; do
    (
        stdout_file=$(realpath --relative-to=. "${output}/$file.stdout")
        stderr_file=$(realpath --relative-to=. "${output}/$file.stderr")

        cargo +stable run "$file" > "$stdout_file" 2> "$stderr_file";

        if [ $? -eq 0 ]; then
            success "Everything is fine for $file"
        else
            failure "Something went wrong for $file, you can find errors at $stderr_file"
        fi
    ) &
done

wait
