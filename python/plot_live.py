import matplotlib.pyplot as plt
from datetime import datetime
import re

LOG_PATTERN = r"Time: ([\d\:\.]+), Sensor: (\d+), Data: ([\d\.]+)"
LOG_FILE = "./sensor_data.log"

def read_sensor_data(filename):
    timestamps = {}
    temperatures = {}
    
    with open(filename, 'r') as f:
        for line in f:
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
    
    return timestamps, temperatures

def plot_sensor_data(timestamps, temperatures):
    plt.figure(figsize=(12, 6))
    
    for sensor in timestamps:
        plt.plot(timestamps[sensor], temperatures[sensor], 
                label=f'Sensor {sensor}', marker='o', markersize=3)
    
    plt.xlabel('Time')
    plt.ylabel('Temperature (°C)')
    plt.title('Sensor Temperature Readings')
    plt.legend()
    plt.grid(True)
    plt.xticks(rotation=45)
    plt.tight_layout()
    
    # Save the plot
    plt.savefig('temperature_plot.png')
    plt.show()

def main():
    try:
        print(f"Reading data from {LOG_FILE}")
        timestamps, temperatures = read_sensor_data(LOG_FILE)
        
        if not timestamps:
            print("No data found in the log file")
            return
            
        print("Plotting sensor data...")
        plot_sensor_data(timestamps, temperatures)
        
    except FileNotFoundError:
        print(f"Error: Could not find log file at {LOG_FILE}")
    except Exception as e:
        print(f"An error occurred: {str(e)}")

if __name__ == "__main__":
    main()