#!/bin/sh

jpm build
mdz
mv site /tmp
git checkout gh-pages
mv /tmp/site/* .
git add . && git commit -m "Update the docs" && git push
git checkout master
rm -rf /tmp/site
