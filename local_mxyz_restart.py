import os
import time

# restart local maderxyz server

# 1. get process ID
os.system('ps aux | grep local_mxyz >> server_process_id.txt')
with open('./server_process_id.txt') as fp:
    lines = fp.readlines()
    first_line = lines[0]
    words = [i for i in first_line.split(' ') if i]
    process_id = words[1]
    print(f'  get server process id: {process_id}')
    os.system('rm ./server_process_id.txt')

# 2. kill server
os.system(f'kill {process_id}')
print('  killing process)')

# 3. restart server using script
os.system('python3 ./local_mxyz_run.py >> /dev/null &')
print('  restarted successfully')
