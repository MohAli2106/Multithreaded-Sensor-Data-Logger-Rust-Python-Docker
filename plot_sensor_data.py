import matplotlib.pyplot as plt
from datetime import datetime
import re
import time
import os
import signal
import sys

LOG_PATTERN = r"Time: ([\d\:\.]+), Sensor: (\d+), Data: ([\d\.]+)"

timestamps = {}
temperatures = {}

plt.ion()
fig, ax = plt.subplots(figsize=(10, 6))

def update_plot():
    ax.clear()
    for sensor in timestamps:
        ax.plot(timestamps[sensor], temperatures[sensor], label=f"Sensor {sensor}")
    ax.set_xlabel("Timestamp")
    ax.set_ylabel("Temperature (°C)")
    ax.set_title("Sensor Temperatures Over Time (Live)")
    ax.legend()
    fig.autofmt_xdate()
    plt.draw()
    plt.pause(0.01)

def follow_logfile(filename):
    with open(filename, "r") as f:
        f.seek(0, os.SEEK_END)
        while True:
            line = f.readline()
            if not line:
                time.sleep(0.2)
                continue
            match = re.match(LOG_PATTERN, line)
            if match:
                tstr, sensor, temp = match.groups()
                sensor = int(sensor)
                temp = float(temp)
                dt = datetime.strptime(tstr, "%H:%M:%S.%f")
                if sensor not in timestamps:
                    timestamps[sensor] = []
                    temperatures[sensor] = []
                timestamps[sensor].append(dt)
                temperatures[sensor].append(temp)
                update_plot()

def handle_signal(sig, frame):
    print("Plotting script terminated by Rust logger.")
    plt.close('all')
    sys.exit(0)

if __name__ == "__main__":
    signal.signal(signal.SIGTERM, handle_signal)
    signal.signal(signal.SIGINT, handle_signal)
    print("Live plotting sensor_data.log. Close the plot window or stop Rust logger to exit.")
    follow_logfile("sensor_data.log")