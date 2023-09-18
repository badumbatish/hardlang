# Write-up documentation

The project is structured in 3 parts in src/

The reader/ folder is for implementation of reader.

The scanner/ folder is for implementation of scanner.

The parser/ folder is for implementation of parser.

## Reader
Currently the reader accepts a file and store all of the file's characters as a string.

Pretty inefficient I know, but I just want to get started with the project. To design a language that's being written with 1 mil+ LOC, I probably need to have a pointer to each file only, and not the whole string. 

This is also why I want to seperate the reader from the lexer, so that the lexer can just called the function provided by the reader and not worry about its implementation.


## Scanner (Lexer)
The lexer's job is provide a function called **lex** that will output a Token to the parser. It does this by calling
from the reader's function calls

## Parser 

TBD