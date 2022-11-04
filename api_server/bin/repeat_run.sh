echo "usage: ./repeat_run.sh [log_file]"
echo "\tif there's no log file, stdout is used instead"
log_counter=0
while :
do
	if [ $# -eq 0 ]
	then
		echo "Running ./api_server"
		./api_server
	else
		echo "Running ./api_server - log file is $1"
		./api_server > $1 2>&1
		logfile="$1_$log_counter"
		mv log "$logfile"
		log_counter=$((log_counter+1))
	fi
	echo "./api_server finished"
	echo "./api_server will start again in 60s"
	sleep 60
done