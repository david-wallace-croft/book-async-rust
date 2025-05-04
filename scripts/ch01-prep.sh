#!/bin/bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

cd $SCRIPTPATH

cd ..

cargo build --bin ch01-connection --release

cargo build --bin ch01-server --release

cp target/release/ch01-connection.exe ./

cp target/release/ch01-server.exe ./
