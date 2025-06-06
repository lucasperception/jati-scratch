import numpy as np
import matplotlib.pyplot as plt
from scipy.interpolate import PchipInterpolator

n_short = np.array([10, 20, 30, 40, 50, 60, 70, 80, 90, 100])
n_full = np.array([10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200,300,400])

t_no_opt = np.array([5.86,24.75,154.7,861.21,1069.26,1653.43,2945.14,4147.88,5598.12,12329.23])
t_opt = np.array([3.75,8.43,40.23,12.06,39.93,82.97,112.79,218.62,241.66,415.88,3657.89,11792.34,32068.42])

no_opt = PchipInterpolator(n_short, t_no_opt)
opt = PchipInterpolator(n_full, t_opt)

n_space_short = np.linspace(min(n_short), max(n_short), 1000)
n_space_long = np.linspace(min(n_full), max(n_full), 1000)

interpol_no_opt = no_opt(n_space_short)
interpol_opt = opt(n_space_long)

plt.figure(figsize=(10, 6))
plt.plot(n_short, t_no_opt, 'o', label='Reference', color='blue')
plt.plot(n_full, t_opt, 'o', label='Optimized', color='orange')
plt.plot(n_space_short, interpol_no_opt, '--', color='blue', alpha=0.7)
plt.plot(n_space_long, interpol_opt, '--', color='orange', alpha=0.7)

plt.xlabel('Number of Taxa')
plt.ylabel('Time (Seconds)')
plt.title('Runtime with fixed sequence length (500) and variable number of taxa')
plt.legend()
plt.grid(True)

plt.show()