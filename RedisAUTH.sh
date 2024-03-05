#!/bin/bash
# This script generates a random password of 18 ASCII characters
# It uses /dev/urandom as a source of randomness
# It filters out non-printable characters and trims the output to 18 characters
# It prints the password to the standard output

password=$(cat /dev/urandom | tr -dc '[:print:]' | head -c 18)
echo $password
