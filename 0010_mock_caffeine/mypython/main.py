"""
Python solution for Caffeine challenge. This is specific for Windows ATM

https://stackoverflow.com/questions/11906925/python-simulate-keydown

Windows API documentation
https://learn.microsoft.com/en-gb/windows/win32/api/winuser/nf-winuser-keybd_event?redirectedfrom=MSDN

https://github.com/moses-palmer/pynput/tree/master/lib/pynput/keyboard
"""

import time
import ctypes

A = 0x41

def simulate_keypress_windows(key_code):
    ctypes.windll.user32.keybd_event(key_code, 0, 0, 0)
    time.sleep(0.1)
    ctypes.windll.user32.keybd_event(key_code, 0, 2, 0)

# TODO: Add check for OS, make sure it is windows. 

import os
import struct

# Define the event codes for key events
EV_KEY = 0x01
KEY_A = 30  # The key code for 'A'

# Define the event structure format
event_format = 'llHHi'
event_size = struct.calcsize(event_format)

# Find the appropriate input event device (replace 'X' with the device number)
input_device = '/dev/input/eventX'

# Open the input event device for writing
with open(input_device, 'wb') as f:
    # Create a keypress event (KEY_A down)
    event = struct.pack(event_format, int(time.time()), 0, EV_KEY, KEY_A, 1)
    f.write(event)
    
    # Sleep to simulate the keypress
    time.sleep(0.1)
    
    # Create a key release event (KEY_A up)
    event = struct.pack(event_format, int(time.time()), 0, EV_KEY, KEY_A, 0)
    f.write(event)
