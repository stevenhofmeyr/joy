#!/usr/bin/bash

left_addr=`bluetoothctl devices|grep "Joy-Con (L)"|cut -d' ' -f2`
right_addr=`bluetoothctl devices|grep "Joy-Con (R)"|cut -d' ' -f2`
bluetoothctl connect $left_addr
bluetoothctl connect $right_addr
