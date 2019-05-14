# gstreak
A command line utility to push commits in a scheduled way.

## Commands

1. `gstreak push`
   
   Pushes the latest changes which are scheduled to be pushed till now. 
This includes the commits not committed using gstreak too.

2.  `gstreak list`

    Lists all commits committed by gstreak but not pushed till now.
   
3. `gstreak commit -m <message> -t "<date expr in natural language>"`

   For ex.
   
   `gstreak commit -m "Fix Readme" -t "2 hours"`
   
   This commits a message which can be pushed later.
   
4. `gstreak check`
    
   Prints the next commit to be pushed
   
## Scheduling
 
Add a crontab entry to `gstreak push` with repo as working directory.