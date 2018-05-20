#!/usr/bin/env sh
# Ghetto parallellism for the win.

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

for file in $(dirname "$0")/*.ö; do
    (
        cargo +stable run "$file" > /dev/null 2> /dev/null;
        if [ $? -eq 0 ]; then
            echo "${GREEN}[✓] Everything is fine for $file.${NC}"
        else
            echo "${RED}[✗] Stuff went wrong for $file.${NC}"
        fi
    ) &
done

wait
