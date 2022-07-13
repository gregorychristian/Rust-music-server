# Rust-music-server
The aim of this project is to have an end to end music server project, much like mopidy, however with the addition of an audio visualizer.
A music server much like mopidy written in rust (backend) and javascript (fronted) with an audio visualizer. Currently only the LED colour picker is working.

Backend:
The backend is written using the Rust actix framework and a serialport library. The server connects to an arudino over USB-Serial and sends strings of Hex code to change the colour of a WS2812B LED strip. The frontend sends HEX strings over WebSocket.

Frontend:
