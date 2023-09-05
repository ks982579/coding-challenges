"""
Python solution for Caffeine challenge. This is specific for Windows ATM

https://stackoverflow.com/questions/11906925/python-simulate-keydown

Windows API documentation
https://learn.microsoft.com/en-gb/windows/win32/api/winuser/nf-winuser-keybd_event?redirectedfrom=MSDN
"""

import time
import ctypes

A = 0x41

def simulate_keypress(key_code):
    ctypes.windll.user32.keybd_event(key_code, 0, 0, 0)
    time.sleep(0.1)
    ctypes.windll.user32.keybd_event(key_code, 0, 2, 0)

# TODO: Add check for OS, make sure it is windows. 

simulate_keypress(A)