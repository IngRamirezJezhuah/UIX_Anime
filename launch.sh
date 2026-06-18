#!/bin/bash

eww kill 

DIR="$(dirname "$0")"

eww -c "$DIR/Front" open main_window

