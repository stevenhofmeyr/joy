#!/usr/bin/env python

# need to run this script as root

import os
import time
import nxbt
import argparse


def connect_switch(pairing):
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

    return nx, controller

def relay_inputs(nx, controller):
    print('Testing button press A')
    nx.press_buttons(controller, [nxbt.Buttons.A], down=1.0)

    FIFO_NAME = 'joycons'
    os.mkfifo(FIFO_NAME)
    os.chmod(FIFO_NAME, mode=0x666)
    print('Waiting for input on pipe', FIFO_NAME)

    try:
        f = open(FIFO_NAME)
        while True:
            data = f.read().strip()
            if len(data) == 0:
                continue
            line = '{0}'.format(data)
            line = line.strip()
            print(line)
            if line == 'quit':
                print('Done with controller')
                break
            buttons = line.split('.')
            for button in buttons:
                if button == '':
                    continue
                if button == 'A':
                    nx.press_buttons(controller, [nxbt.Buttons.A], down=0.5)
                elif button == 'B':
                    nx.press_buttons(controller, [nxbt.Buttons.B], down=0.5)
                elif button == 'X':
                    nx.press_buttons(controller, [nxbt.Buttons.X], down=0.5)
                elif button == 'Y':
                    nx.press_buttons(controller, [nxbt.Buttons.Y], down=0.5)
                elif button == 'DPAD_UP':
                    nx.press_buttons(controller, [nxbt.Buttons.DPAD_UP], down=0.5)
                elif button == 'DPAD_DOWN':
                    nx.press_buttons(controller, [nxbt.Buttons.DPAD_DOWN], down=0.5)
                elif button == 'DPAD_LEFT':
                    nx.press_buttons(controller, [nxbt.Buttons.DPAD_LEFT], down=0.5)
                elif button == 'DPAD_RIGHT':
                    nx.press_buttons(controller, [nxbt.Buttons.DPAD_RIGHT], down=0.5)
                elif button == 'L':
                    nx.press_buttons(controller, [nxbt.Buttons.L], down=0.5)
                elif button == 'ZL':
                    nx.press_buttons(controller, [nxbt.Buttons.ZL], down=0.5)
                elif button == 'R':
                    nx.press_buttons(controller, [nxbt.Buttons.R], down=0.5)
                elif button == 'ZR':
                    nx.press_buttons(controller, [nxbt.Buttons.ZR], down=0.5)
                elif button == 'JCL_SL':
                    nx.press_buttons(controller, [nxbt.Buttons.JCL_SL], down=0.5)
                elif button == 'JCL_SR':
                    nx.press_buttons(controller, [nxbt.Buttons.JCL_SR], down=0.5)
                elif button == 'L_STICK_PRESS':
                    nx.press_buttons(controller, [nxbt.Buttons.L_STICK_PRESS], down=0.5)
                elif button == 'R_STICK_PRESS':
                    nx.press_buttons(controller, [nxbt.Buttons.R_STICK_PRESS], down=0.5)
                elif button == 'MINUS':
                    nx.press_buttons(controller, [nxbt.Buttons.MINUS], down=0.5)
                elif button == 'PLUS':
                    nx.press_buttons(controller, [nxbt.Buttons.PLUS], down=0.5)
                elif button == 'CAPTURE':
                    nx.press_buttons(controller, [nxbt.Buttons.CAPTURE], down=0.5)
                elif button == 'HOME':
                    nx.press_buttons(controller, [nxbt.Buttons.HOME], down=0.5)
                else:
                    print('Unrecognized input:', button)
    finally:
        os.remove(FIFO_NAME)

    # when done, free adapter
    nx.remove_controller(controller)

if __name__ == "__main__":
    argparser = argparse.ArgumentParser(add_help=True)
    argparser.add_argument('--pair', action='store_true',
                        help='Explicit pairing to connect to an unpaired switch the first time around')
    options = argparser.parse_args()
    nx, controller = connect_switch(options.pair)
    if options.pair:
        print('Connected for first time. Need to rerun in order to be able to use joycons too.')
    else:
        relay_inputs(nx, controller)
