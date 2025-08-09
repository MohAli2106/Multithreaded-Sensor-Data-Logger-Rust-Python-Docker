# python/app.py
from flask import Flask, send_from_directory
import os
import sys
import time
import threading

# Project root
PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.append(PROJECT_ROOT)

# Import plot generator
from sensor_plot import generate_plot

STATIC_DIR = os.path.join(PROJECT_ROOT, "python", "static")

app = Flask(__name__)

def update_plot_periodically():
    while True:
        generate_plot()  # Read fresh file, plot, save
        time.sleep(5)    # Every 5 seconds

@app.route('/')
def index():
    return '''
    <html>
        <head>
            <title>Sensor Dashboard</title>
            <meta http-equiv="refresh" content="5">
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; }
                img { border: 1px solid #ddd; border-radius: 8px; width: 100%; }
            </style>
        </head>
        <body>
            <h1>🌡️ Real-Time Sensor Monitoring</h1>
            <p><strong>Last updated:</strong> ''' + time.strftime("%H:%M:%S") + '''</p>
            <img src="/plot.png" alt="Live Plot">
        </body>
    </html>
    '''

@app.route('/plot.png')
def plot():
    return send_from_directory(STATIC_DIR, "plot.png")

if __name__ == '__main__':
    os.makedirs(STATIC_DIR, exist_ok=True)
    thread = threading.Thread(target=update_plot_periodically, daemon=True)
    thread.start()
    app.run(host='0.0.0.0', port=5000)