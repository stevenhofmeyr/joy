#!/usr/bin/env python

# need to run this script as root

import os
import sys
import time
import argparse

import nxbt
import nxbt.tui


nxbt_buttons = [b for b in dir(nxbt.Buttons) if not b.startswith("__")]


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


def process_ringcon(nx, controller, input_val):
    fields = input_val.split(",")
    if len(fields) > 0 and fields[0] != "RING":
        return False
    flex = fields[1]
    # flex ranges from min 350 to max 4840, with resting value of 2540
    print("FLEX", flex, "%.2f" % ((float(flex) - 2540) / 2300))
    return True


def process_movement(nx, controller, input_val):
    fields = input_val.split(",")
    if len(fields) > 0 and fields[0] != "ROT" and fields[0] != "ACC":
        return False
    side, x, y, z = fields[1:]
    print(fields[0], side, x, y, z)
    return True


def process_stick(nx, controller, input_val):
    fields = input_val.split(",")
    if len(fields) > 0 and fields[0] != "STICK":
        return False
    side = fields[1]
    x = float(fields[2])
    y = float(fields[3])
    print("STICK", side, x, y)
    nx.tilt_stick(controller, side[0] + "_STICK", int(100 * x), int(100 * y), block=False)
    return True


def process_buttons(tui, input_val, pressed_buttons, new_pressed_buttons):
    fields = input_val.split(",")
    if len(fields) > 0 and fields[0] != "BUTTONS":
        return False
    for button in fields[1:]:
        if button in nxbt_buttons:
            if pressed_buttons.pop(button, None) == None:
                print("on_press({})".format(button))
                if tui is not None:
                    tui.on_press(button)
            new_pressed_buttons[button] = True
        else:
            print("Unrecognized input:", button)
    return True


def relay_inputs(tui):
    FIFO_NAME = "joycons"
    os.mkfifo(FIFO_NAME)
    os.chmod(FIFO_NAME, mode=0o666)

    if tui is not None:
        tui.direct_input_loop(tui.term, run_loop=False)

    pressed_buttons = {}
    print("Waiting for input on pipe", FIFO_NAME)
    try:
        f = open(FIFO_NAME, "r")
        done = False
        while not done:
            data = f.readline()
            if len(data) == 0:
                continue
            input_vals = "{0}".format(data).split(" ")
            # print(input_vals)
            sys.stdout.flush()
            new_pressed_buttons = {}
            for input_val in input_vals:
                input_val = input_val.strip()
                if "quit" in input_val:
                    print("Done with controller")
                    done = True
                    break
                process_buttons(tui, input_val, pressed_buttons, new_pressed_buttons)
                # process_stick(nx, controller, input_val)
                # process_movement(nx, controller, input_val)
                # process_ringcon(nx, controller, input_val)
            # clear out buttons that are no longer pressed
            for button in pressed_buttons:
                print("on_release({})".format(button))
                if tui is not None:
                    tui.on_release(button)
            # keep track of all currently pressed buttons
            pressed_buttons = new_pressed_buttons
            print("currently pressed buttons", pressed_buttons)

    finally:
        os.remove(FIFO_NAME)
        if tui is not None:
            tui.shutdown_direct_loop()


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
        relay_inputs(None)
    else:
        nx, controller = reconnect_switch()
        tui = nxbt.tui.InputTUI()
        relay_inputs(tui)
        nx.remove_controller(controller)
