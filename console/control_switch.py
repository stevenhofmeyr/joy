#!/usr/bin/env python

# need to run this script as root

import multiprocessing
import os
import sys
import time
import argparse

import nxbt


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


def process_ringcon(console_packet, input_val, log_f):
    flex = input_val.split(",")[1]
    # flex ranges from min 350 to max 4840, with resting value of 2540
    print("FLEX", flex, "%.2f" % ((float(flex) - 2540) / 2300), file=log_f, flush=True)


def process_movement(console_packet, input_val, log_f):
    fields = input_val.split(",")
    side, x, y, z = fields[1:]
    print(fields[0], side, x, y, z, file=log_f, flush=True)


def process_stick(console_packet, input_val, log_f):
    fields = input_val.split(",")
    side = fields[1]
    x = int(100 * float(fields[2]))
    y = int(100 * float(fields[3]))
    if abs(x) > 10 or abs(y) > 10:
        print("STICK", side, x, y, file=log_f, flush=True)
    packet = console_packet["packet"]
    packet[side[0] + "_STICK"]["X_VALUE"] = x
    packet[side[0] + "_STICK"]["Y_VALUE"] = y
    console_packet["packet"] = packet


def process_buttons(console_packet, input_val, pressed_buttons, new_pressed_buttons, log_f):
    button = input_val.split(",")[1]
    if button in nxbt_buttons:
        if pressed_buttons.pop(button, None) == None:
            print("press button", button, file=log_f, flush=True)
            # the input packet is a synchronized object between processes, so we need to set it this way to ensure atomicity
            # we don't need anything more sophisticated because the input_worker just reads the shared object and never sets it
            packet = console_packet["packet"]
            if button in ["L_STICK_PRESS", "R_STICK_PRESS"]:
                packet[button[0:7]]["PRESSED"] = True
            else:
                packet[button] = True
            console_packet["packet"] = packet
        new_pressed_buttons[button] = True
    else:
        print("Unrecognized input:", button)


def release_buttons(console_packet, pressed_buttons, log_f):
    # clear out buttons that are no longer pressed
    for button in pressed_buttons:
        print("release button", button, file=log_f, flush=True)
        packet = console_packet["packet"]
        if button in ["L_STICK_PRESS", "R_STICK_PRESS"]:
            packet[button[0:7]]["PRESSED"] = False
        else:
            packet[button] = False
        console_packet["packet"] = packet


def console_worker(nx, controller, console_packet):
    if controller is None:
        f = open("console_worker.log", "w")
    while True:
        packet = console_packet["packet"]
        if controller is not None:
            nx.set_controller_input(controller, packet)
            # this is the frequency expected for a pro controller
            time.sleep(1.0 / 120)
        else:
            print(packet, file=f)
            f.flush()
            # slow frequency for easy debugging
            time.sleep(5)


def relay_inputs(nx, controller):
    FIFO_NAME = "joycons"
    while True:
        try:
            os.mkfifo(FIFO_NAME)
            os.chmod(FIFO_NAME, mode=0o666)
            break
        except FileExistsError:
            os.remove(FIFO_NAME)

    packet_manager = multiprocessing.Manager()
    console_packet = packet_manager.dict()
    console_packet["packet"] = nx.create_input_packet()
    console_process = multiprocessing.Process(target=console_worker, args=(nx, controller, console_packet))
    console_process.start()

    pressed_buttons = {}
    print("Waiting for input on pipe", FIFO_NAME)
    log_f = open("connection.log", "w")
    try:
        f = open(FIFO_NAME, "r")
        done = False
        while not done:
            data = f.readline()
            if len(data) == 0:
                continue
            input_vals = "{0}".format(data).strip().split(" ")
            # print(input_vals, file=log_f, flush=True)
            sys.stdout.flush()
            new_pressed_buttons = {}
            for input_val in input_vals:
                input_val = input_val.strip()
                if "quit" in input_val:
                    print("Done with controller")
                    done = True
                    break
                input_type = input_val.split(",")[0]
                if input_type == "BUTTON":
                    process_buttons(console_packet, input_val, pressed_buttons, new_pressed_buttons, log_f)
                elif input_type == "STICK":
                    process_stick(console_packet, input_val, log_f)
                elif input_type == "ROT" or input_type == "ACC":
                    process_movement(console_packet, input_val, log_f)
                elif input_type == "FLEX":
                    process_ringcon(console_packet, input_val, log_f)
                else:
                    print("unknown input type", input_type, file=log_f, flush=True)
            release_buttons(console_packet, pressed_buttons, log_f)
            pressed_buttons = new_pressed_buttons
            if len(pressed_buttons) > 0:
                print("currently pressed buttons", pressed_buttons, file=log_f, flush=True)

    finally:
        os.remove(FIFO_NAME)
        if nx is not None:
            packet_manager.shutdown()
            console_process.terminate()


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
        nx = nxbt.Nxbt()
        relay_inputs(nx, None)
    else:
        nx, controller = reconnect_switch()
        relay_inputs(nx, controller)
        nx.remove_controller(controller)
