#!/bin/bash
# filepath: /Users/kevinthomas/Documents/embassy/examples/rp/update_fork.sh

# Set variables
UPSTREAM_REPO="https://github.com/embassy-rs/embassy.git"
BRANCH="main"

# Add upstream remote if it doesn't exist
if ! git remote | grep -q upstream; then
    echo "Adding upstream repository..."
    git remote add upstream $UPSTREAM_REPO
fi

# Fetch changes from upstream
echo "Fetching changes from upstream..."
git fetch upstream

# Merge changes into the current branch
echo "Merging changes from upstream/$BRANCH into local $BRANCH..."
git checkout $BRANCH
git merge upstream/$BRANCH

# Push changes to your fork
echo "Pushing changes to your fork..."
git push origin $BRANCH

echo "Your fork is now up-to-date with the upstream repository!"
