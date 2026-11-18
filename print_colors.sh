#!/usr/bin/env zsh

for i in {0..15}; do
    printf '\033[38;5;%dm FG %02d \033[48;5;%dm BG %02d \033[0m\n' \
        "$i" "$i" "$i" "$i"
done