# mader.xyz

This repo contains the code running [my website](mader.xyz).

## Installation

To contribute, you need to install the server locally via the following steps:

- download the repo

        git clone https://github.com/vincentmader/mader.xyz

- make sure you have python3 and pip installed

- create a virtual python environment 

        cd mader.xyz
        virtualenv venv

- install python dependecies

        pip3 install -r requirements.txt

- test-run the server

        cd FlaskApp
        python3 __init__.py

- head to your browser (localhost:5000) and check if it works

## Setup

After completing the installation, should take a look at the file

    ./FlaskApp/config.py
    
and initialize the path variables.

