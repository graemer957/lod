#!/bin/zsh
set -e

# shellcheck disable=SC2088
DEST_DIR="~/code/$(basename "$PWD")"

# Remember to use '' to stop local shell expansion
rsync -a --force --del --delete --delete-excluded -e ssh --exclude='*/.git' --exclude='target' . switch:"$DEST_DIR"

# Execute tarpaulin remotely
ssh -t switch "cd $DEST_DIR && cargo tarpaulin -o Html"

# Copy back the report for inspection locally
scp -q switch:"$DEST_DIR"/tarpaulin-report.html .

# Last updated: 20231111
