#!/bin/zsh

set -e

COUNT=100

fastly compute build

LOG=/tmp/viceroy.log
rm -f $LOG
echo "Starting server"
viceroy serve -v --log-stdout target/wasm32-wasi/release/fastly-compute-project.wasm > $LOG &
while ! grep -q 'Listening on' $LOG;
do
    sleep 0.1
done
echo "Started server, making $COUNT requests"
repeat $COUNT do curl -s http://127.0.0.1:7878/ >/dev/null; done
echo "Finished, killing server"
kill %1

avg=$(cat $LOG | grep 'Generated CAPTCHA in:' | sed -E -e 's/.*Generated CAPTCHA in: ([0-9]+)$/\1/' | awk '{ sum += $0; count += 1 } END { print sum/count }')
echo "Average generation time: $avg ms"
