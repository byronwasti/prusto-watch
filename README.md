*Note: This project is a work in progress*

# OpenWatch

Open source smart watch.

## Design

## Components
- Battery
    - Name: RJD3032
    - digikey: 1572-1622-ND
    - V = 3.7V
    - Capacity = 200mAh
    - I (max) = 40mA
- Screen (SPI + Enable)  
    - Name: LS010B7DH01
    - digikey: 425-2899-ND
    - P (max) = 55uW (still) or 100uW (change)
    - V = 4.5-5.5V
    - I (max) = 11uA (still) or 20uA (change)
- Screen Connector (FPC)
    - Name: Molex LLC 5034801000 
    - digikey: WM1389CT-ND 
    - NOTE: Need to verify that this is the right part
- Charge Pump (3.3V -> 5V for Screen)
    - Name: TPS60241DGKR
    - digikey: 296-12241-6-ND 
    - I (out) = 25mA
    - I (quiescent) = 250-400uA (1uA in shutdown)
- Microcontroller 
    - Name: STM32F303RCT6
    - digikey: 497-13304-ND
    - V = 3.3V
    - I (max) = 160mA
    - I (typ) = ~20mA
    - I (PVD) = 0.15uA (nominal)
- IMU (SPI)
    - Name: LSM6DSLTR
    - digikey: 497-16705-1-ND 
    - V = 1.7 - 3.6V (1.8V typ)
    - I (gyro norm) = 0.45mA
    - I (acc norm) = 85uA
    - I (power down) = 3uA
- 4 Capacitive touch sensors (4 I/O)
    - Name: AT42QT1070-MMHR
    - digikey: AT42QT1070-MMHCT-ND 
    - V = 3.3V
    - I (@3.3V) = 434uA - 906uA
    - I (standalone) = 615uA
- Bluetooth (SPI)
    - Name: BM71BLES1FC2-0002AA
    - digikey: BM71BLES1FC2-0002AA-ND 
    - V = 3.3V
    - I (rx/tx) = 10mA (typ) 13mA (max)
    - I (reduced) = 60uA (typ)
    - I (shutdown) = 1uA (min) 2.9uA (max)
- External flash (SPI) ???
- Dual channel NAND Gate
    - Name: SN74LVC2G132DCUR
    - digikey: 296-18802-1-ND 
    - V = 3.3V
    - I (quiescent) = 10uA (max)
- Vibration motor
    - Name: Adafruit Industries LLC 1201 
    - digikey: 1528-1177-ND 
    - V = 2-5V
    - I (@3V) = 60mA
- MOSFET (to drive vibration motor)
    - name: RUM002N02T2L
    - digikey: RUM002N02T2LCT-ND 
    - Vgs = 8V (max) 1V (max thresh)
- Flyback Diode ???
    - name: ON Semiconductor BAT54 
    - digikey: BAT54FSCT-ND 

