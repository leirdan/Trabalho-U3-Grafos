#!/bin/sh

for f in data/*; do
    fname=$(basename $f)
    num=$(echo $fname | grep -o -E '[1-9]+[0-9]*')
    sed -i "s|\(graph_from_csv!(\"\)[^\"]*\(\")\)|\1$f/data.csv\2|" ./src/bin/nearest_insertion.rs
    echo -n "instance $num: "
    cargo rr --bin nearest_insertion 2> /dev/null
done
