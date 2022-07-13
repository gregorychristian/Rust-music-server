# Rust-music-server
The aim of this project is to have an end to end music server project, much like mopidy, however with the addition of an audio visualizer.
A music server much like mopidy written in rust (backend) and javascript (fronted) with an audio visualizer. Currently only the LED colour picker is working.

<h1>Backend</h1>
The backend is written using the Rust actix framework and a serialport library. The server connects to an arudino over USB-Serial and sends strings of Hex code to change the colour of a WS2812B LED strip. The frontend sends HEX strings over WebSocket.

<h1>Frontend</h1>
<img
  src="https://raw.githubusercontent.com/gregorychristian/Rust-music-server/main/Frontend.jpeg"
  alt="Alt text"
  title="Optional title"
  style="display: inline-block; margin: 0 auto; max-width: 300px">

The frontend is currently written in Javascript (iro.js library), CSS and HTML. Clicking on the visulizer tab button will reveal the iro.js Colour Picker. A TypeScript file explorer will be added later.

<h1>Features</h1>

-[x] Add Colour Picker
-[ ] TypeScript file Explorer
-[ ] Music Player
-[ ] Audio visualizer customisation
