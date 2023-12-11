import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import scienceplots

# Import parquet file
df = pd.read_parquet('c1.parquet')

# Prepare Data to Plot
y = []
for i in df.columns:
    y.append(df[i])

x = np.linspace(0, 1, len(y[0]))

# Plot params
pparam = dict(
    xlabel = r'$\phi$',
    ylabel = r'$V(\phi)$',
    xscale = 'linear',
    yscale = 'linear',
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    for i in range(len(y)):
        ax.plot(x, y[i], alpha=0.7)
    fig.savefig('plot.png', dpi=600, bbox_inches='tight')
