#!/bin/sh

# build code first
jpm build

# stash changes if it exist
git stash

# site generation
mdz
mv site /tmp
git checkout gh-pages
mv /tmp/site/* .

# Commit changes and go back to main branch
git add . && git commit -m "Update the docs" && git push
git checkout master
rm -rf /tmp/site
