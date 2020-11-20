# mader.xyz

This repo contains the code running [my website](http://mader.xyz).

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
  (if not, continue with the next section and check again afterwards)

## Setup

After completing the installation, you should take a look at the file

    ./FlaskApp/config.py
    
and initialize the path variables. You also need to create the file

    ./.env

and define your application secret key (used for encryption). 
This file is loaded from 

    ./flaskapp.wsgi

and should look like this:

    SECRET_KEY="<your secret key goes here>"
    
