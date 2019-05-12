# gstreak
A command line utility to push commits in a scheduled way.

## Commands

`gstreak push`
   
   Pushes the latest changes which are scheduled to be pushed till now. 
This includes the commits not committes using gstreak too.


`gstreak list`

   Lists all commits committed by gstreak but not pushed till now.
   
`gstreak commit -m <message> -d "<date-exp>"`

   For ex.
   
   `gstreak commit -m "Fix Readme" -d "now+2h"`
   
   
## Scheduling
 
Add a crontab entry to `gstreak push` with repo as working directory.