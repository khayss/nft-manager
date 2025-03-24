#!/bin/bash

# Command to upgrade the IDL
anchor idl upgrade 78TGdayzTnEPi8UVMeRgJYSx6uawNB3CHTrcBBMM2gDK -f target/idl/nft_manager.json --provider.cluster Devnet

# Check if the command was successful
if [ $? -eq 0 ]; then
    echo "IDL upgrade successful."
else
    echo "IDL upgrade failed."
    exit 1
fi