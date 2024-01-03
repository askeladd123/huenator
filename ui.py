import socket
import json
import tkinter as tk
from threading import Thread

ADDRESS = "127.0.0.1"
PORT = 8000

def rgb_to_hex(rgb):
    return "#{:02x}{:02x}{:02x}".format(*rgb)

def update_ui(color_frame, debug_label, rgb_colors, debug_text):
    for i, color in enumerate(rgb_colors):
        hex_color = rgb_to_hex(color)
        if i < len(color_frame.winfo_children()):
            color_frame.winfo_children()[i].config(bg=hex_color)
        else:
            label = tk.Label(color_frame, bg=hex_color)
            label.pack(side='left', fill='both', expand=True)

    for i in range(len(rgb_colors), len(color_frame.winfo_children())):
        color_frame.winfo_children()[i].destroy()

    debug_label.config(text=debug_text, bg="black", fg="white", font=("Helvetica", 16))
    debug_label.pack(side='bottom', fill='x')

def udp_listener(window, color_frame, debug_label):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((ADDRESS, PORT))

    while True:
        data, addr = sock.recvfrom(1024)
        try:
            data_json = json.loads(data.decode())  
            rgb_colors = data_json.get('rgb_colors', [])
            debug_text = data_json.get('debug_message', "")
            window.after(0, update_ui, color_frame, debug_label, rgb_colors, debug_text)
        except json.JSONDecodeError:
            print("Received invalid JSON")

def create_window():
    window = tk.Tk()
    window.title("Multiple Color Display")
    window.geometry("600x300")  

    color_frame = tk.Frame(window)
    color_frame.pack(side='top', fill='both', expand=True)

    debug_label = tk.Label(window, text="", bg="black", fg="white", font=("Helvetica", 16))
    debug_label.pack(side='bottom', fill='x')

    Thread(target=udp_listener, args=(window, color_frame, debug_label)).start()
    window.mainloop()

create_window()
