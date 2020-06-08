# TORAT
TORAT (Treasury Office Routing number Analysis Tool) is a tool designed to be used by state governments to track out of state routing numbers to detect potential fraud. For input, the program takes a text file that has the bank routing numbers seperated by a new line. The output file will be a text file that contains all of the routing numbers belonging to out of state banks as well as the name of those banks and which state they blong to.

## Commands/Flags: 
```
help	Display a help message.
-h	Display a help message.
-f	Change the output file.
-i	Change the input file.
-s	Change the state that we are filtering by.
-d	Change the database file we are refrencing. 
-l	Single lookup mode. 
```

## Examples: 
```
torat -h
torat -f out.txt -i target.txt
torat -s NH -d data.csv
torat -d data.csv -l #########
```
## Default values:
- Input file: target.txt.
- Output file: out.txt.
- Default State: ME.
- Database file: data.csv.
