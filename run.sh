#!/usr/bin/sh

START=2.4
END=3.8

TEST=510
CPUS=4

for u in $(seq $START 0.1 $END | sed 's/,/./'); do
        cargo run --release --                  \
                --default-policy SCHED_DEADLINE \
                --calibration 30                \
                --logdir "./rt-app-log/${u}u"   \
                --round-C                       \
                --period-min 20000              \
                --period-max 200000             \
                --period-gran 10000             \
                --num-sets 30                   \
                --num-tasks 16                  \
                --taskset-utilization ${u}

        mv json/Config* json/${TEST}_test/${CPUS}cpus
done
