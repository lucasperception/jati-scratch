import numpy as np
import matplotlib.pyplot as plt
from scipy.interpolate import PchipInterpolator

n_full = np.array([10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200])

t_opt = np.array([1.37, 4.63, 21.2, 33.38, 52.93, 99.29, 141.72, 206.03, 234.88, 416.1, 4080.12])
t_cache = np.array([1.42,4.72,21.56,32.76,52.6,97.74,137.54,196.86,235.14,414.69,4054.67])

no_opt = PchipInterpolator(n_full, t_cache)
opt = PchipInterpolator(n_full, t_opt)

n_space_long = np.linspace(min(n_full), max(n_full), 1000)

interpol_no_opt = no_opt(n_space_long)
interpol_opt = opt(n_space_long)

plt.figure(figsize=(10, 6))
plt.plot(n_full, t_cache, 'o', label='Optimized + Cache Locality', color='blue')
plt.plot(n_full, t_opt, 'o', label='Optimized', color='orange')
plt.plot(n_space_long, interpol_no_opt, '-', color='blue', alpha=0.7)
plt.plot(n_space_long, interpol_opt, '--', color='orange', alpha=0.7)

plt.xlabel('Number of Taxa')
plt.ylabel('Time (Seconds)')
plt.title('Runtime with fixed sequence length (n = 500) and variable number of taxa')
plt.legend()
plt.grid(True)

plt.show()
