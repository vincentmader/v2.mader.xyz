#!/usr/bin/python3

from dotenv import load_dotenv
from FlaskApp import app as application
import os
import sys
import logging
sys.path.insert(0, '/var/www/FlaskApp')
logging.basicConfig(stream=sys.stderr)

load_dotenv()

application.secret_key = os.getenv('$SECRET_KEY', 'for dev')
