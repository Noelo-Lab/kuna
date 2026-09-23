#!/bin/bash
~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-g/tools/speedextra.py 15 /home/mahaloz/kwt/_final-g/speed-extra.json > /home/mahaloz/kwt/_final-g/speed-extra.log 2>&1
~/.virtualenvs/decbench/bin/python /home/mahaloz/kwt/_final-g/tools/speed7.py 11 /home/mahaloz/kwt/_final-g/speed.json > /home/mahaloz/kwt/_final-g/speed.log 2>&1
echo ALLSPEED_DONE
