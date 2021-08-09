# kill flask server instances, if running
pid="$(ps -p $(lsof -ti tcp:5000) o pid=)"
kill $pid

# kill mongo demon instances, if running
killall mongod

# start database & flask server
cd $CODE/mader.xyz
mongod --dbpath=$CODE/mader.xyz/database &
flask run 
# $CODE/mader.xyz/venv/bin/flask run --no-debugger
