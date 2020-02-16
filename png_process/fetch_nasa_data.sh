#!/bin/bash

# you should create a folder and run this script.

# fetch monthly data
wget --no-directories --no-host-directories --no-parent --recursive --mirror --accept "*.PNG" -l1 https://neo.sci.gsfc.nasa.gov/archive/gs/MYD28M/
# fetch weekly data
wget --no-directories --no-host-directories --no-parent --recursive --mirror --accept "*.PNG" -l1 https://neo.sci.gsfc.nasa.gov/archive/gs/MYD28W/
