import socket
import json
import tkinter as tk
from threading import Thread

ADDRESS = "127.0.0.1"
PORT = 8000

def rgb_to_hex(rgb):
    return "#{:02x}{:02x}{:02x}".format(*rgb)

def update_colors(window, colors): 
    for widget in window.winfo_children():
        widget.destroy()

    for color in colors:
        hex_color = rgb_to_hex(color)  # Convert RGB to hex
        label = tk.Label(window, bg=hex_color)
        label.pack(side='left', fill='both', expand=True)

def udp_listener(window):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((ADDRESS, PORT))

    while True:
        data, addr = sock.recvfrom(1024)
        try:
            rgb_colors = json.loads(data.decode())  
            window.after(0, update_colors, window, rgb_colors)
        except json.JSONDecodeError:
            print("Received invalid JSON")

def create_window():
    window = tk.Tk()
    window.title("Multiple Color Display")
    window.geometry("600x300")  

    Thread(target=udp_listener, args=(window,)).start()
    window.mainloop()

create_window()
