#!/bin/bash

eww kill 

DIR="$(dirname "$0")"
CFG_DIR="$DIR/Front"

eww --config "$CFG_DIR" daemon &
sleep 0.1
eww -c "$CFG_DIR" open --toggle main_window

