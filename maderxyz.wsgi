#!/usr/bin/python3
import logging
import sys
import os

from FlaskApp import app as application
from dotenv import load_dotenv

sys.path.insert(0, '/var/www/maderxyz')
sys.path.insert(0, '/var/www/maderxyz/FlaskApp')
logging.basicConfig(stream=sys.stderr)

load_dotenv()
application.secret_key = os.getenv('$SECRET_KEY', 'for dev')
