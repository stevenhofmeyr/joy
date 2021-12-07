#!/usr/bin/env python

# need to run this script as root

import time
import nxbt


# Start the NXBT service
nx = nxbt.Nxbt()

# Get a list of all previously connected Switches
addresses = nx.get_switch_addresses()
# no previous connection
if len(addresses) == 0:
    print('No addresses found, pairing...')
    # Create a Pro Controller and wait for it to connect (only do this the first time)
    controller = nx.create_controller(nxbt.PRO_CONTROLLER)
    nx.wait_for_connection(controller)
    print("Connected to Switch")
else:
    # reconnect to a Switch
    print('Found address', addresses[0])
    # pass the list as a reconnect_address argument
    controller = nx.create_controller(nxbt.PRO_CONTROLLER, reconnect_address=addresses[0])
    print('Reconnected to Switch')

print('Testing button press A')
nx.press_buttons(controller, [nxbt.Buttons.A], down=1.0)

print('Waiting for input from joycons.dat')
f = open('joycons.dat', 'r')    
while True:
    line = f.readline().strip()
    if not line:
        #time.sleep(0.01)
        continue
    print(line)
    if line == 'button.A':
        nx.press_buttons(controller, [nxbt.Buttons.A], down=0.5)
    elif line == 'button.B':
        nx.press_buttons(controller, [nxbt.Buttons.B], down=0.5)
    elif line == 'button.X':
        nx.press_buttons(controller, [nxbt.Buttons.X], down=0.5)
    elif line == 'button.Y':
        nx.press_buttons(controller, [nxbt.Buttons.Y], down=0.5)
    else:
        print('Not implemented:', line)
    
# when done, free adapter
#nx.remove_controller(controller)
