#!/usr/bin/env sh
# Ghetto parallellism for the win.

for file in *; do
    (cargo run "$file" > /dev/null 2> /dev/null;
    if [ $? -eq 0 ]; then
        echo "Everything is fine for $file."
    else
        echo "Stuff went wrong for $file."
    fi) &
done

wait
