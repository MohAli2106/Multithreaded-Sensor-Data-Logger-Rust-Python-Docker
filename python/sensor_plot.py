# python/sensor_plot.py
import matplotlib.pyplot as plt
import os
import re
from datetime import datetime

# Project root (where Docker puts everything)
PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LOG_FILE = os.path.join(PROJECT_ROOT, "sensor_data.log")
STATIC_DIR = os.path.join(PROJECT_ROOT, "python", "static")
PLOT_PATH = os.path.join(STATIC_DIR, "plot.png")

LOG_PATTERN = re.compile(r"Time:\s*([\d:\.]+),\s*Sensor:\s*(\d+),\s*Data:\s*([\d\.]+)")

def generate_plot():
    os.makedirs(STATIC_DIR, exist_ok=True)

    # Read log file EVERY TIME (no caching)
    if not os.path.exists(LOG_FILE):
        _create_waiting_plot("Waiting for sensor data...")
        return

    # 🔁 Read file fresh each time, then close immediately
    lines = []
    try:
        with open(LOG_FILE, "r") as f:
            lines = f.readlines()
    except Exception as e:
        _create_waiting_plot(f"Error reading log: {str(e)}")
        return

    # Parse data
    timestamps = {1: [], 2: []}
    temperatures = {1: [], 2: []}

    for line in lines:
        match = LOG_PATTERN.match(line.strip())
        if match:
            tstr, sensor_id, temp = match.groups()
            sensor = int(sensor_id)
            temp = float(temp)
            try:
                dt = datetime.strptime(tstr, "%H:%M:%S.%f")
                elapsed = (dt - datetime.strptime("00:00:00.000", "%H:%M:%S.%f")).total_seconds()
            except:
                continue

            if sensor not in timestamps:
                timestamps[sensor] = []
                temperatures[sensor] = []

            if elapsed not in timestamps[sensor]:
                timestamps[sensor].append(elapsed)
                temperatures[sensor].append(temp)

    # Plot
    plt.figure(figsize=(12, 6))
    plotted = False
    for sensor in sorted(timestamps):
        ts = timestamps[sensor]
        temp = temperatures[sensor]
        if ts:
            sorted_data = sorted(zip(ts, temp))
            tss, temps = zip(*sorted_data)
            plt.plot(tss, temps, label=f'Sensor {sensor}', marker='o', markersize=3)
            plotted = True

    if not plotted:
        _create_waiting_plot("No valid sensor data yet...")
        return

    plt.xlabel("Time (seconds since start)")
    plt.ylabel("Temperature (°C)")
    plt.title("Real-Time Sensor Monitoring")
    plt.legend()
    plt.grid(True)
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.savefig(PLOT_PATH)
    plt.close()

def _create_waiting_plot(message):
    plt.figure(figsize=(12, 6))
    plt.text(0.5, 0.5, message, fontsize=20, ha='center')
    plt.axis('off')
    plt.tight_layout()
    plt.savefig(PLOT_PATH)
    plt.close()