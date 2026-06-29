#!/bin/bash

for file in *.po; do
  echo "=== Checking $file ==="
  msgcmp "$file" xrayhexgenerator.pot
done
