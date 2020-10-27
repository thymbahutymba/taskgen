#!/usr/bin/sh

START=1.4
END=1.9
 #$*
for u in $(seq $START 0.1 $END | sed 's/,/./'); do
#for u in $(seq $START $END); do
        cargo run --release -- --round-C --default-policy SCHED_DEADLINE --calibration 30 -p 20000 -q 200000 -g 10000 \
        -s 10 \
        -n 10 \
        -u ${u}

for file in json/Config*; do
        echo "$(jq '.global.logdir = "./rt-app-log/'${u}'u"' ${file})" > ${file}
done

mv json/Config* json/504_test/2cpus

done
