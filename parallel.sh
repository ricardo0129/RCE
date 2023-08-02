#!/bin/bash

# Number of times to call the Python script
num_calls=1

i=1
while [ $i -le $num_calls ]; do
    python3 query.py &
    # Store the process ID (PID) of the last background job
done
