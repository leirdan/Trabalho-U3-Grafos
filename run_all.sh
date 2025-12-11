#!/bin/sh

for f in data/*; do
    fname=$(basename $f)
    num=$(echo $fname | grep -o -E '[1-9]+[0-9]*')
    echo ">> Running instance $num"
    sed -i "s|\(graph_from_csv!(\"\)[^\"]*\(\")\)|\1$f/data.csv\2|" ./src/bin/memetic.rs
    cargo br
    ./run.sh ./results/memetic/$fname/result.txt 1422 162 0.0193
    ./parse_result.py ./results/memetic/$fname/result.txt > ./results/memetic/$fname/summary.txt
    echo "Summary:"
    cat ./results/memetic/$fname/summary.txt
done

