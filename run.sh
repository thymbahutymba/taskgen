#!/usr/bin/sh

cargo run --release -- --round-C --default-policy SCHED_DEADLINE -p 30000 -q 200000 -g 10000 $*
