#!/usr/bin/env bash

# Serve the application on localhost:4000 using Python 2 or 3.

IS_PYTHON_2=$(python -V | grep "2.")
IS_PYTHON_3=$(python -V | grep "3.")

if [[ $IS_PYTHON_2 ]]; then
  echo "Serving index.html on localhost:4000 using Python 2 SimpleHTTPServer. Press CTRL-C to stop."
  python -m SimpleHTTPServer 4000
fi

if [[ $IS_PYTHON_3 ]]; then
  echo "Serving index.html on localhost:4000 using Python 3 http.server. Press CTRL-C to stop."
  python -m http.server 4000
fi
