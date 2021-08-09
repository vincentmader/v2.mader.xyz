# DEACTIVATE RUNNING SERVER SCRIPTS (if running)
# =============================================================================

# kill flask server instances
pid="$(ps -p $(lsof -ti tcp:5000) o pid=)"
kill $pid

# kill mongo demon instances
killall mongod

# TODO: kill frontend instances (in a less brutal way)
killall node

# deactivate virtualenv
deactivate

# RESTART BACKEND, FRONTEND & DATABASE
# =============================================================================

# move into project directory
cd $CODE/mader.xyz

# TODO: launch virtualenv
# source venv/bin/activate

# start database & flask server
mongod --dbpath=$CODE/mader.xyz/database &
flask run &
# $CODE/mader.xyz/venv/bin/flask run --no-debugger

# start frontend
cd $CODE/mader.xyz/FlaskApp/static/js/react_frontend
npm start
