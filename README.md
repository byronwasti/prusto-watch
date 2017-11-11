*Note: This project is a work in progress*

# OpenWatch

Open source smart watch.

## Design
[TODO]

## Components

### Power Management
- Battery
    - Name: RJD3032
    - digikey: 1572-1622-ND
    - V = 3.7V
    - Capacity = 200mAh
    - I (max) = 40mA
    - V (charge) = 4.2V
    - V (cutoff) = 3.0V
- Battery Clip (holder)
    - Name: BH3000
    - digikey: BH3000-ND 
    - Height: ~9mm
- Battery Charger
    - Name: MCP73831-2ATI/MC 
    - digikey: MCP73831-2ATI/MC-ND  
    - V = 4.2V
    - I = 100mA (prog=10K)
- Power multiplexer
    - Name: TPS2113ADRBR
    - digikey: 296-46373-1-ND 
    - I (quiescent) = 55uA
- Voltage regulator (High power)
    - Name: TPS70933DBVR 
    - digikey: 296-35483-1-ND  
    - V = 3.3V
    - I (quiescent) = 1uA
    - I (out) = 150mA

### Screen
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
- Boost Converter (3.3V -> 5V for screen power) [ NOT USED ]
    - Name: TPS61222DCKR
    - digikey: 296-39435-1-ND 

### MCU
- Microcontroller 
    - Name: STM32F303C8T6 
    - digikey: 497-15198-ND  
    - V = 3.3V
    - I (max) = 160mA
    - I (typ) = ~20mA
    - I (PVD) = 0.15uA (nominal)
- 16MHz Crystal
    - Name: FA-238 16.0000MB-C3 
    - digikey: SER3686CT-ND 
    - Load Capacitance: 18pF
- 32.768 RTC
    - Name: ECS-.327-12.5-34B-TR 
    - digikey: XC1617TR-ND 
    - Load Capacitance: 12.5pF
- 3 Color LED
    - Name: SMLP34RGB2W3 
    - digikey: SMLP34RGB2W3CT-ND 

### IMU
- IMU (SPI)
    - Name: LSM6DSLTR
    - digikey: 497-16705-1-ND 
    - V = 1.7 - 3.6V (1.8V typ)
    - I (gyro norm) = 0.45mA
    - I (acc norm) = 85uA
    - I (power down) = 3uA

### Capactive Touch
- 4 Capacitive touch sensors (4 I/O)
    - Name: AT42QT1070-MMHR
    - digikey: AT42QT1070-MMHCT-ND 
    - V = 3.3V
    - I (@3.3V) = 434uA - 906uA
    - I (standalone) = 615uA

### BLE
- Bluetooth (UART)
    - Name: BM71BLES1FC2-0002AA
    - digikey: BM71BLES1FC2-0002AA-ND 
    - V = 3.3V
    - I (rx/tx) = 10mA (typ) 13mA (max)
    - I (reduced) = 60uA (typ)
    - I (shutdown) = 1uA (min) 2.9uA (max)

- Bluetooth (UART)
    - Name: RN4871-V
    - digikey: RN4871-V/RM118-ND 
    - V = 3.3V
    - I (rx/tx) = 10mA (typ) 13mA (max)
    - I (reduced) = 60uA (typ)
    - I (shutdown) = 1uA (min) 2.9uA (max)

- Brown Out Detection
    - Name: RT9818A-18GV
    - digikey: RT9818A-18GV-ND 
    - V (threshold) = 1.8V
    - I (quiescent) = 3uA

### Vibration Motor
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

### Extra
- External flash (SPI) [ NOT SPECCED ]

## Power Budget
Power budget is the maximum average power over an hour.
- MCU = 20mA
- Power Management = 55uA + 1uA ~ 60uA
- Screen = 15uA + 400uA ~  450uA
- IMU = 450uA + 85uA ~ 600uA
- Touch = 615uA
- BLE = ~10mA
- Vibration = ~10mA

Sum = 20mA + 0.06mA + 0.45mA + 0.6mA + 0.6mA + 10mA + 10mA ~ 42mA

Giving us a little under 0.25C draw rate, or about 4 hours of battery life minimum.

Taking into account the lower power modes of various peripherals, we can assume: BLE=60uA, IMU=3uA, and Vibration=0.1mA (pulsed, and not very often). This gives us an average current draw of:

20mA + 0.06mA + 0.45mA + 0.003mA + 0.06mA + 0.6mA + 0.1mA = 21mA => 0.12C draw rate ~ 10hrs

Ideally we can reduce the power consumption of our MCU even more, giving us extra buffer room for other components.

