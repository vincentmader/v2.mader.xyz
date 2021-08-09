#!/usr/bin/python3

import logging
logging.basicConfig(stream=sys.stderr)

import sys
sys.path.insert(0, '/var/www/maderxyz')
sys.path.insert(0, '/var/www/maderxyz/FlaskApp')
from FlaskApp import app as application

import os
from dotenv import load_dotenv
load_dotenv()
application.secret_key = os.getenv('$SECRET_KEY', 'for dev')
# application.secret_key = 'my secret key'
