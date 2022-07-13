
function Show(){
  var element = document.getElementById("picker");
  element.style.display = 'block';
}
function Hide(){
  var element = document.getElementById("picker");
  element.style.display = 'none';
}
var colorPicker = new iro.ColorPicker("#picker", {
  width: 500,
  color: "rgb(225, 0, 0)",
  borderWidth: 1,
  borderColor: "#fff",
  });
var wsUri = (window.location.protocol=='https:'&&'wss://'||'ws://')+window.location.host + '/ws/';
conn = new WebSocket(wsUri);
console.log('Connecting...');
conn.onopen = function() {
    console.log('Connected.');
    };
// listen to a color picker's color:change event
// color:change callbacks receive the current color
colorPicker.on('color:change', function(color) {
    // log the current color as a HEX string
    conn.send(color.hexString);
  });


