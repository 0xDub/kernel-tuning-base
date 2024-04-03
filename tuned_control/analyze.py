import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import seaborn as sns
import random

plt.style.use('dark_background')

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    DIM = '\033[2m'
    UNDERLINE = '\033[4m'


def analyze():
    threshold = 30000000000000 # in microseconds


    available_colors = ["#21fced", "#21fc80", "#fc2130", "#219efc", "#ed21fc", "#8021fc", "#30fc21", "#fc8021", "#FFFF00", "#FDECA0", "#FF00FF", "#00FFFF", "#FF0000", "#00FF00", "#0000FF", "#FF8000", "#FF0080", "#80FF00", "#80FF00", "#0080FF", "#8000FF", "#FF8080", "#80FF80", "#8080FF", "#FF80FF", "#80FFFF", "#FF80FF", "#FFFF80", "#FF8000", "#FF0080", "#80FF00", "#80FF00", "#0080FF", "#8000FF", "#FF8080", "#80FF80", "#8080FF", "#FF80FF", "#80FFFF", "#FF80FF", "#FFFF80", "#FF8000", "#FF0080", "#80FF00", "#80FF00", "#0080FF", "#8000FF", "#FF8080", "#80FF80", "#8080FF", "#FF80FF", "#80FFFF", "#FF80FF", "#FFFF80", "#FF8000", "#FF0080", "#80FF00", "#80FF00", "#0080FF", "#8000FF", "#FF8080", "#80FF80", "#8080FF", "#FF80FF", "#80FFFF", "#FF80FF", "#FFFF80", "#FF8000", "#FF0080", "#80FF00", "#80FF00", "#0080FF", "#8000FF", "#FF8080", "#80FF80", "#8080FF", "#FF80FF", "#80FFFF", "#FF80FF", "#FFFF80", "#FF8000", "#FF0080", "#80FF00", "#80FF00", "#0080FF", "#8000FF", "#FF8080", "#80FF80", "#8080FF", "#FF80FF", "#80FFFF", "#FF80FF", "#FFFF80", "#FF8000", "#FF0080", "#80FF00", "#80FF00", "#0080FF", "#8000FF", "#FF8080", "#80FF80", "#8080FF", "#FF80FF", "#80FFFF", "#FF80FF", "#FFFF80", "#FF8000", "#FF0080", "#80FF00", "#80FF00", "#0080FF", "#8000FF", "#FF8080", "#80FF80", "#8080FF"]

    methods = ["SlowNoData", "BurstNoData", "ConsistentNoData", "SlowWithData", "BurstWithData", "ConsistentWithData", "SlowLargeData", "BurstLargeData", "ConsistentLargeData"] # BurstLargeData

    aggregate_data = pd.DataFrame(columns=["method", "latency"])

    print()
    print(bcolors.HEADER + "=-=-= Kernel Tuning | Latency Stats (us) =-=-=" + bcolors.ENDC)

    fig, comp_ax = plt.subplots(1)
    comp_ax.set_title("Kernel Tuning | Latency Distribution")
    comp_ax.set_xlabel("Latency (us)")
    comp_ax.set_ylabel("Density")
    print()

    for method in methods:

        with open(f"tuned/{method}.txt", "r") as f:
            lines = f.readlines()
            lines = [x.rstrip() for x in lines]
            lines = [int(x) for x in lines if x != ""]
        tuned_data = pd.DataFrame(lines, columns=["latency"])
        tuned_data = tuned_data[tuned_data["latency"] < threshold * 1000]
        tuned_data["method"] = method
        tuned_data["server"] = "tuned"
        if len(aggregate_data) == 0:
            aggregate_data = tuned_data
        else:
            aggregate_data = pd.concat([aggregate_data, tuned_data])


        with open(f"control/{method}.txt", "r") as f:
            lines = f.readlines()
            lines = [x.rstrip() for x in lines]
            lines = [int(x) for x in lines if x != ""]
        control_data = pd.DataFrame(lines, columns=["latency"])
        control_data = control_data[control_data["latency"] < threshold * 1000]
        control_data["method"] = method
        control_data["server"] = "control"
        if len(aggregate_data) == 0:
            aggregate_data = control_data
        else:
            aggregate_data = pd.concat([aggregate_data, control_data])


        tuned_mean = round(tuned_data["latency"].mean() / 1000)
        tuned_std = round(tuned_data["latency"].std() / 1000)
        control_mean = round(control_data["latency"].mean() / 1000)
        control_std = round(control_data["latency"].std() / 1000)

        tuned_mean_perf = round((tuned_mean / control_mean) * 100 - 100)
        tuned_std_perf = round((tuned_std / control_std) * 100 - 100)

        print(f"=---------= {method} =---------=")
        print(f"Tuned       | Mean: {tuned_mean} | Std: {tuned_std}")
        print(f"Control     | Mean: {control_mean} | Std: {control_std}")
        print(bcolors.OKCYAN + f"Performance | Mean: {tuned_mean_perf}% | Std: {tuned_std_perf}%" + bcolors.ENDC)
        print()

        if method == "ConsistentNoData" or method == "ConsistentWithData" or method == "ConsistentLargeData":
            print(bcolors.HEADER + "=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=" + bcolors.ENDC)
            print()
        

        bins = 1000
        alpha = 0.25

        # get random color from available colors
        color = available_colors.pop(0)
        comp_ax.hist(tuned_data["latency"], bins=bins, alpha=alpha, density=True, color=color, label=method)
        comp_ax.axvline(tuned_data["latency"].mean(), color=color, linestyle='dashed', linewidth=1)

        comp_ax.hist(control_data["latency"], bins=bins, alpha=alpha, density=True, color=color, label=method)
        comp_ax.axvline(control_data["latency"].mean(), color=color, linestyle='solid', linewidth=1)

    comp_ax.legend()
    plt.show()



analyze()