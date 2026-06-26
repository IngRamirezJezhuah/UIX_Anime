#!/bin/bash

# Controlador para Eww
case $1 in
    "open_player")
        eww open main_window
        ;;
    "open_dashboard")
        eww update active_module="dashboard"
        ;;
    "player_prev")
        # Comando para ir a la canción anterior
        playerctl previous
        ;;
    "player_next")
        # Comando para ir a la siguiente canción
        playerctl next
        ;;
    "player_toggle")
        # Comando para pausar/reproducir
        playerctl play-pause
        ;;
    *)
        echo "Comando no reconocido"
        ;;
esac
