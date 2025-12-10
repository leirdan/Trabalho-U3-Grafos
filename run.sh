#!/bin/sh

DURATION=60
END_TIME=$(($(date +%s) + $DURATION))
TEMP_DIR=/tmp/grasp
RESULT=${1:-result.txt}
if [ -n "$1" ]; then shift 1; fi
PARAMS=${@:-"100 100 0.01"}

rm -rf $TEMP_DIR
mkdir -p $TEMP_DIR

for i in $(seq $(nproc)); do
    TEMP_RESULT=$TEMP_DIR/result_$i.txt
    while [ $(date +%s) -lt $END_TIME ]; do
        # TODO: change this to the desired bin depending on the algorithm
        ./target/release/graphs-algorithms $PARAMS >> $TEMP_RESULT
    done &
done

wait

cat $TEMP_DIR/result_*.txt > $RESULT
