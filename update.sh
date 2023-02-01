#!/bin/sh
cd "${0%/*}"
git pull
mdbook build
