import multiplex
import time
import board
import adafruit_tca9548a
import adafruit_ssd1306

multiplex.I2C_setup(0x70,0)


import board


# Create I2C bus as normal

i2c = board.I2C()  # uses board.SCL and board.SDA


tca = adafruit_tca9548a.TCA9548A(i2c)

d1 = adafruit_ssd1306.SSD1306_I2C(128, 64, tca[0])
d2 = adafruit_ssd1306.SSD1306_I2C(128, 32, tca[1])

d1.fill(1)
d2.fill(1)

d1.show()
d2.show()

d1.fill(0)
d2.fill(0)

d1.show()
d2.show()



