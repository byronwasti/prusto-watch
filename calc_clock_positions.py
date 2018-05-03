import numpy as np
import matplotlib.pyplot as plt

radius = 60
offset = (65, 65)
divisions = 60

def calc_pixels(radius, offset, divisions):
    rads = [ (i * (np.pi/30)) for i in range(divisions) ]
    func_x = lambda i: int(radius * np.cos(i)) + offset[0]
    func_y = lambda i: int(radius * np.sin(i)) + offset[1]
    vals_x = [ func_x(i) for i in rads]
    vals_y = [ func_y(i) for i in rads]
    return vals_x, vals_y

x, y = calc_pixels(radius, offset, divisions)
plt.plot(x[15:30], y[15:30])
#plt.show()
print(list(zip(x, y)))

