#!/bin/bash

compiler=$1
file=$2
output=$3
#addtionalArg=$4


exec  1> "logfile"
exec  2> "errors"

START=$(date +%s.%2N)
#Branch 1
if [ "$output" = "" ]; then
    $compiler ./$file -< "inputFile" #| tee /usercode/output.txt
#Branch 2
else
	#In case of compile errors, redirect them to a file
        $compiler ./$file $addtionalArg #&> /usercode/errors.txt
	#Branch 2a
	if [ $? -eq 0 ];	then
		$output -< "inputFile" #| tee /usercode/output.txt    
	#Branch 2b
	else
	    echo "Compilation Failed"
	    #if compilation fails, display the output file	
	    #cat /usercode/errors.txt
	fi
fi

#exec 1>&3 2>&4

#head -100 /usercode/logfile.txt
touch completed
END=$(date +%s.%2N)
runtime=$(echo "$END - $START" | bc)

mv logfile completed
chmod o+w *

shutdown
