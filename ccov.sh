#!/bin/bash
set -e

SERVER=$1
if [ -z "${SERVER}" ]; then
	echo 'Need to specify server name! eg, `./ccov.sh apoc`'
	exit 1;
fi

DEST_DIR="~/code/$(basename "$PWD")"

# Remember to use '' to stop local shell expansion
rsync -a --force --del --delete --delete-excluded -e ssh --exclude='*/.git' --exclude='target' . $SERVER:"$DEST_DIR"

# Execute tarpaulin remotely
ssh -t $SERVER ". .cargo/env && cd $DEST_DIR && cargo tarpaulin -o Html"

# Copy back the report for inspection locally
scp -q $SERVER:"$DEST_DIR"/tarpaulin-report.html .

# Last updated: 20231202
