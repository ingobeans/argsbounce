# argsbounce

simple executable that logs all times it's executed and with what args. 

i made this when doing some malware analysis of a heavily obfuscated batch script. the script created several child processes of cmd.exe and powershell.exe, and i used this to intercept the args they were sent. i just made two copies of this executable and renamed to cmd.exe and powershell.exe.

it logs to your user profile, in a new dir called argsbounce. each time the executable is called it will create a new subdirectory there named with the corresponding run index. in that directory it logs a `.full.txt` which contains all args concatenated, as well as an individual txt file for each arg passed.

ex. C:/Users/tea/argsbounce/3/.full.txt
