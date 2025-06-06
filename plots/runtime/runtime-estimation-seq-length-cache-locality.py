import numpy as np
import matplotlib.pyplot as plt
from scipy.interpolate import PchipInterpolator

n = np.array([1000, 5000, 10000, 20000, 50000, 100000, 200000])
t_cache = np.array([2.93,17.14,31.04,67.29,319.19,470.58,2248.84])
t_opt = np.array([2.93,17.33,31.92,71.66,332.89,527.16,2437.15])

no_opt = PchipInterpolator(n, t_cache)
opt = PchipInterpolator(n, t_opt)

n_space = np.linspace(min(n), max(n), 1000)

interpol_no_opt = no_opt(n_space)
interpol_opt = opt(n_space)

plt.figure(figsize=(10, 6))
plt.plot(n, t_cache, 'o', label='Optimized + Cache Locality', color='blue')
plt.plot(n, t_opt, 'o', label='Optimized', color='orange')
plt.plot(n_space, interpol_no_opt, '-', color='blue', alpha=0.7)
plt.plot(n_space, interpol_opt, '--', color='orange', alpha=0.7)

plt.xlabel('Sequence Length (Base pairs)')
plt.ylabel('Time (Seconds)')
plt.title('Runtime with fixed number of taxa (10) with variable sequence lengths')
plt.legend()
plt.grid(True)

plt.show()
