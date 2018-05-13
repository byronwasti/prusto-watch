import numpy as np
import matplotlib.pyplot as plt

radius = 55
offset = (65, 65)
divisions = 12

def calc_pixels(radius, offset, divisions):
    rads = [ (i * (np.pi/(divisions/2))) for i in range(divisions) ]
    func_x = lambda i: int(radius * np.cos(i)) + offset[0]
    func_y = lambda i: int(radius * np.sin(i)) + offset[1]
    vals_x = [ func_x(i) for i in rads]
    vals_y = [ func_y(i) for i in rads]
    return vals_x, vals_y

x, y = calc_pixels(radius, offset, divisions)
plt.plot(x, y)
#plt.show()
print(list(zip(x, y)))

