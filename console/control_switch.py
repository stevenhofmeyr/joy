#!/usr/bin/env python

# need to run this script as root

import os
import sys
import time
import nxbt
import argparse


def pair_switch():
    # Init
    print("[1] Attempting to initialize NXBT...")
    nx = None
    try:
        nx = nxbt.Nxbt(pairing=True)
    except Exception as e:
        print("Failed to initialize:", e)
        exit(1)
    print("Successfully initialized NXBT.\n")
    # Adapter Check
    print("[2] Checking for Bluetooth adapter availability...")
    adapters = None
    try:
        adapters = nx.get_available_adapters()
    except Exception as e:
        print("Failed to check for adapters:", e)
        exit(1)
    if len(adapters) < 1:
        print("Unable to detect any Bluetooth adapters.")
        print("Please ensure you system has Bluetooth capability.")
        exit(1)
    print(f"{len(adapters)} Bluetooth adapter(s) available.")
    print("Adapters:", adapters, "\n")
    # Creating a controller
    print("[3] Please turn on your Switch and navigate to the 'Change Grip/Order menu.'")
    input("Press Enter to continue...")
    print("Creating a controller with the first Bluetooth adapter...")
    cindex = None
    try:
        cindex = nx.create_controller(
            nxbt.PRO_CONTROLLER,
            adapters[0],
            colour_body=[255, 0, 0],
            colour_buttons=[255, 0, 0],
        )
    except Exception as e:
        print("Failed to create a controller:", e)
        exit(1)
    print("Successfully created a controller.\n")
    # Controller connection check
    print("[4] Waiting for controller to connect with the Switch...")
    timeout = 120
    print(f"Connection timeout is {timeout} seconds for this test script.")
    elapsed = 0
    while nx.state[cindex]["state"] != "connected":
        if elapsed >= timeout:
            print("Timeout reached, exiting...")
            exit(1)
        elif nx.state[cindex]["state"] == "crashed":
            print("An error occurred while connecting:")
            print(nx.state[cindex]["errors"])
            exit(1)
        elapsed += 1
        time.sleep(1)
    print("Successfully connected.\n")


def reconnect_switch():
    # Start the NXBT service
    nx = nxbt.Nxbt(pairing=False)

    # Get a list of all previously connected Switches
    addresses = nx.get_switch_addresses()

    # no previous connection
    if len(addresses) == 0:
        print("No addresses found, pairing...")
        # Create a Pro Controller and wait for it to connect (only do this the first time)
        controller = nx.create_controller(nxbt.PRO_CONTROLLER)
        nx.wait_for_connection(controller)
        print("Connected to Switch")
    else:
        # reconnect to a Switch
        print("Found address", addresses[0])
        # pass the list as a reconnect_address argument
        controller = nx.create_controller(nxbt.PRO_CONTROLLER, reconnect_address=addresses[0])
        print("Reconnected to Switch")

    return nx, controller


def process_ringcon(nx, controller, input):
    fields = input.split(",")
    if len(fields) > 0 and fields[0] != "RING":
        return False
    flex = fields[1]
    # flex ranges from min 350 to max 4840, with resting value of 2540
    print("FLEX", flex, "%.2f" % ((float(flex) - 2540) / 2300))
    return True


def process_movement(nx, controller, input):
    fields = input.split(",")
    if len(fields) > 0 and fields[0] != "ROT" and fields[0] != "ACC":
        return False
    side, x, y, z = fields[1:]
    print(fields[0], side, x, y, z)
    return True


def process_stick(nx, controller, input):
    fields = input.split(",")
    if len(fields) > 0 and fields[0] != "STICK":
        return False
    side = fields[1]
    x = float(fields[2])
    y = float(fields[3])
    print("STICK", side, x, y)
    nx.tilt_stick(controller, side[0] + "_STICK", int(100 * x), int(100 * y), block=False)
    return True


def process_buttons(nx, controller, input):
    fields = input.split(",")
    if len(fields) > 0 and fields[0] != "BUTTONS":
        return False
    DOWN = 0.1
    UP = 0.1
    BLOCK = False
    for button in fields[1:]:
        if nx is None:
            print("BUTTON", button)
            sys.stdout.flush()
            continue
        if button == "A":
            nx.press_buttons(controller, [nxbt.Buttons.A], down=DOWN, up=UP, block=BLOCK)
        elif button == "B":
            nx.press_buttons(controller, [nxbt.Buttons.B], down=DOWN, up=UP, block=BLOCK)
        elif button == "X":
            nx.press_buttons(controller, [nxbt.Buttons.X], down=DOWN, up=UP, block=BLOCK)
        elif button == "Y":
            nx.press_buttons(controller, [nxbt.Buttons.Y], down=DOWN, up=UP, block=BLOCK)
        elif button == "DPAD_UP":
            nx.press_buttons(controller, [nxbt.Buttons.DPAD_UP], down=DOWN, up=UP, block=BLOCK)
        elif button == "DPAD_DOWN":
            nx.press_buttons(controller, [nxbt.Buttons.DPAD_DOWN], down=DOWN, up=UP, block=BLOCK)
        elif button == "DPAD_LEFT":
            nx.press_buttons(controller, [nxbt.Buttons.DPAD_LEFT], down=DOWN, up=UP, block=BLOCK)
        elif button == "DPAD_RIGHT":
            nx.press_buttons(controller, [nxbt.Buttons.DPAD_RIGHT], down=DOWN, up=UP, block=BLOCK)
        elif button == "L":
            nx.press_buttons(controller, [nxbt.Buttons.L], down=DOWN, up=UP, block=BLOCK)
        elif button == "ZL":
            nx.press_buttons(controller, [nxbt.Buttons.ZL], down=DOWN, up=UP, block=BLOCK)
        elif button == "R":
            nx.press_buttons(controller, [nxbt.Buttons.R], down=DOWN, up=UP, block=BLOCK)
        elif button == "ZR":
            nx.press_buttons(controller, [nxbt.Buttons.ZR], down=DOWN, up=UP, block=BLOCK)
        elif button == "JCL_SL":
            nx.press_buttons(controller, [nxbt.Buttons.JCL_SL], down=DOWN, up=UP, block=BLOCK)
        elif button == "JCL_SR":
            nx.press_buttons(controller, [nxbt.Buttons.JCL_SR], down=DOWN, up=UP, block=BLOCK)
        elif button == "L_STICK_PRESS":
            nx.press_buttons(controller, [nxbt.Buttons.L_STICK_PRESS], down=DOWN, up=UP, block=BLOCK)
        elif button == "R_STICK_PRESS":
            nx.press_buttons(controller, [nxbt.Buttons.R_STICK_PRESS], down=DOWN, up=UP, block=BLOCK)
        elif button == "MINUS":
            nx.press_buttons(controller, [nxbt.Buttons.MINUS], down=DOWN, up=UP, block=BLOCK)
        elif button == "PLUS":
            nx.press_buttons(controller, [nxbt.Buttons.PLUS], down=DOWN, up=UP, block=BLOCK)
        elif button == "CAPTURE":
            nx.press_buttons(controller, [nxbt.Buttons.CAPTURE], down=DOWN, up=UP, block=BLOCK)
        elif button == "HOME":
            nx.press_buttons(controller, [nxbt.Buttons.HOME], down=DOWN, up=UP, block=BLOCK)
        else:
            print("Unrecognized input:", button)
    return True


def relay_inputs(nx, controller):
    FIFO_NAME = "joycons"
    os.mkfifo(FIFO_NAME)
    os.chmod(FIFO_NAME, mode=0o666)

    print("Waiting for input on pipe", FIFO_NAME)
    try:
        f = open(FIFO_NAME, "r")
        while True:
            data = f.readline()
            if len(data) == 0:
                continue
            inputs = "{0}".format(data).split(" ")
            print(inputs)
            sys.stdout.flush()
            for input in inputs:
                input = input.strip()
                if "quit" in input:
                    print("Done with controller")
                    break
                process_buttons(nx, controller, input)
                process_stick(nx, controller, input)
                process_movement(nx, controller, input)
                process_ringcon(nx, controller, input)

    finally:
        os.remove(FIFO_NAME)
        if nx:
            nx.remove_controller(controller)


if __name__ == "__main__":
    argparser = argparse.ArgumentParser(add_help=True)
    argparser.add_argument(
        "--pair",
        action="store_true",
        help="Explicit pairing to connect to an unpaired switch the first time around",
    )
    argparser.add_argument(
        "--test-inputs",
        action="store_true",
        help="Test the parsing of the inputs, without needing a connection to the switch",
    )
    options = argparser.parse_args()
    if options.pair:
        pair_switch()
    if options.test_inputs:
        relay_inputs(None, None)
    else:
        nx, controller = reconnect_switch()
        relay_inputs(nx, controller)
