import plasma
import random
from plasma import plasma2040
import time
import machine


## start listening to a serial connection as a slave

NUM_LEDS = 100
led_strip = plasma.WS2812(NUM_LEDS, 0, 0, plasma2040.DAT, color_order=plasma.COLOR_ORDER_RGB)
led_strip.start()

def set_rgb(r,g,b):
    for i in range(NUM_LEDS):
        led_strip.set_rgb(i, r, g, b)

def set_hsv(h,s,v):
    for i in range(NUM_LEDS):
        led_strip.set_hsv(i, h, s, v)
