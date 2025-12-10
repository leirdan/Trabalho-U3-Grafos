#!/bin/sh

for f in data/*; do
    fname=$(basename $f)
    num=$(echo $fname | grep -o -E '[1-9]+[0-9]*')
    echo ">> Running instance $num"
    sed -i "s|\(graph_from_csv!(\"\)[^\"]*\(\")\)|\1$f/data.csv\2|" ./src/main.rs
    cargo br
    if [ $((num % 2)) = 0 ]; then
        ./run.sh ./data/$fname/result.txt 2452 197 0.0193
    else
        ./run.sh ./data/$fname/result.txt 677  195 0.0152
    fi
    ./parse_result.py ./data/$fname/result.txt > ./data/$fname/summary.txt
    echo "Summary:"
    cat ./data/$fname/summary.txt
done
