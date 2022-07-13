#include <FastLED.h>

#define NUM_LEDS 144

#define DATA_PIN 6

char rx_byte = 0;
String rx_str = "";
// Define the array of leds
CRGB leds[NUM_LEDS];

void setup() {
  Serial.begin(9600);
  FastLED.addLeds<NEOPIXEL, DATA_PIN>(leds, NUM_LEDS);  // GRB ordering is assumed
}


void loop() { 

    if (Serial.available() > 0){
      rx_byte = Serial.read();  
      rx_str += rx_byte;
    if (strlen(rx_str.c_str()) == 6)  {
      fill_solid(leds, NUM_LEDS, strtol(rx_str.c_str(), NULL, 16));
      FastLED.show();
      rx_str = "";
    }
  }
}
