#!/bin/bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

cd $SCRIPTPATH

cd ..

./ch01-server.exe &

pid1=$!

./ch01-server.exe &

pid2=$!

./ch01-server.exe &

pid3=$!

./ch01-server.exe &

pid4=$!

wait $pid1

exit_code1=$?

wait $pid2

exit_code2=$?

wait $pid3

exit_code3=$?

wait $pid4

exit_code4=$?

echo "Task 1 (PID $pid1) exited with code $exit_code1"
echo "Task 1 (PID $pid2) exited with code $exit_code2"
echo "Task 1 (PID $pid3) exited with code $exit_code3"
echo "Task 1 (PID $pid4) exited with code $exit_code4"
