#
# host machine pings a configuration endpoint, receives a JSON config
# passes the config over serial to the LED driver

# ideally: import a python module that allows you to ping the config
# import a python module that allows you to update the lamp

# define a python module that accepts a dictionary of brightness and color
# and then passes those values over serial to a microcontroller

import serial
import webcolors
import colorsys

# write a module that instantiates with a serial address (like COM5)
# and then exposes different methods to control the lamp
# like set_brightness, set_color, set_color_temp, etc

class Pixels:
    def __init__(self, comport):
        self.comport = comport
        self.ser = serial.Serial(comport, 9600, timeout=1)
        self.brightness = 1.0
        self.saturation = 1.0
        self.hue = 0.0

    def set_color(self, color):
        rgb = webcolors.name_to_rgb(color)
        hsv = colorsys.rgb_to_hsv(rgb[0]/255, rgb[1]/255, rgb[2]/255)
        self.hue = (hsv[0] + 0.5) % 1
        self.saturation = hsv[1]
        self.brightness = hsv[2]
        self.set_hsv()

    def set_hue(self, hue):
        self.hue = hue
        self.set_hsv()
    
    def set_saturation(self, saturation):
        self.saturation = saturation
        self.set_hsv()
    
    def set_brightness(self, brightness):
        self.brightness = brightness
        self.set_hsv()

    def set_rgb(self, r, g, b):
        try:
            self.ser.write(f"set_rgb({r}, {g}, {b})\r\n".encode())
            response = self.ser.readline().decode().strip()
            print(f"Response: {response}")
        except Exception as e:
            print(f"Error sending command: {e}")

    def set_hsv(self):
        try:
            self.ser.write(f"set_hsv({self.hue}, {self.saturation}, {self.brightness})\r\n".encode())
            response = self.ser.readline().decode().strip()
            print(f"Response: {response}")
        except Exception as e:
            print(f"Error sending command: {e}")

    def done(self):
        self.ser.close()