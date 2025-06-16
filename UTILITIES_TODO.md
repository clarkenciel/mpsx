# POSIX Utilities Implementation TODO

This document lists POSIX utilities that could be implemented in this project, ordered from least complex to most complex. The complexity assessment is based on the number of options, algorithmic complexity, system interactions, and overall implementation difficulty.

## Status Legend
- ✅ **Implemented**: Already completed
- 🚧 **In Progress**: Currently being worked on  
- ⭐ **High Priority**: Good next candidates
- 📝 **Medium Priority**: Moderate complexity
- 🔧 **Advanced**: Complex system interactions
- 🏗️ **Expert Level**: Very complex, requires deep system knowledge

---

## Level 1: Basic Utilities (Minimal Complexity)

### ✅ Implemented
- **wc** - Word, line, character, and byte count (via `mwc` crate)

### ⭐ High Priority (Good Next Steps)
- **true** - Return successful exit status
- **false** - Return unsuccessful exit status  
- **echo** - Display a line of text
- **pwd** - Print working directory
- **basename** - Strip directory and suffix from filenames
- **dirname** - Strip non-directory suffix from file name
- **yes** - Output a string repeatedly until killed
- **sleep** - Suspend execution for an interval

### 📝 Medium Priority  
- **cat** - Concatenate and print files
- **tee** - Read from input and write to output and files
- **head** - Output the first part of files
- **tail** - Output the last part of files
- **touch** - Change file timestamps
- **mkdir** - Make directories
- **rmdir** - Remove empty directories

---

## Level 2: Text Processing (Low-Medium Complexity)

### ⭐ High Priority
- **tr** - Translate or delete characters
- **cut** - Extract selected fields from each line
- **uniq** - Report or omit repeated lines
- **nl** - Number lines of files
- **expand** - Convert tabs to spaces
- **unexpand** - Convert spaces to tabs

### 📝 Medium Priority
- **sort** - Sort lines of text files
- **fold** - Wrap each input line to fit specified width
- **join** - Join lines of two files on a common field
- **paste** - Merge lines of files
- **comm** - Compare two sorted files line by line
- **split** - Split a file into pieces
- **csplit** - Split a file based on context lines

---

## Level 3: File Operations (Medium Complexity)

### ⭐ High Priority
- **cp** - Copy files or directories
- **mv** - Move (rename) files
- **rm** - Remove files and directories
- **ln** - Make links between files
- **chmod** - Change file mode bits
- **chown** - Change file ownership
- **chgrp** - Change group ownership

### 📝 Medium Priority
- **ls** - List directory contents
- **du** - Display directory space usage
- **df** - Display filesystem disk space usage
- **file** - Determine file type
- **cksum** - Display file checksums
- **cmp** - Compare two files byte by byte
- **dd** - Convert and copy a file

---

## Level 4: System Interaction (Medium-High Complexity)

### 📝 Medium Priority
- **env** - Run a program in a modified environment
- **id** - Print real and effective user and group IDs  
- **uname** - System information
- **date** - Display or set the system date
- **cal** - Display a calendar
- **who** - Show who is logged on
- **logname** - Print user's login name

### 🔧 Advanced
- **ps** - Report a snapshot of current processes
- **kill** - Send a signal to a process
- **nohup** - Run commands immune to hangups
- **nice** - Run a command with modified priority
- **renice** - Alter priority of running processes
- **time** - Time a simple command
- **timeout** - Run a command with a time limit

---

## Level 5: Advanced Text Processing (High Complexity)

### 🔧 Advanced
- **grep** - Search text patterns in files
- **sed** - Stream editor for filtering and transforming text
- **diff** - Compare files line by line
- **patch** - Apply changes to files
- **strings** - Print the sequences of printable characters in files
- **od** - Dump files in octal and other formats

### 🏗️ Expert Level
- **awk** - Pattern scanning and processing language
- **m4** - Macro processor
- **lex** - Generate lexical analyzer
- **yacc** - Yet Another Compiler Compiler

---

## Level 6: System Administration (High Complexity)

### 🔧 Advanced
- **find** - Search for files and directories
- **xargs** - Build and execute command lines from standard input
- **stty** - Change and print terminal line settings
- **test** - Evaluate conditional expressions
- **expr** - Evaluate expressions
- **getopts** - Parse utility options

### 🏗️ Expert Level
- **tar** - Archive files (though often provided as `pax`)
- **pax** - Portable archive interchange
- **crontab** - Schedule tasks for execution
- **at** - Schedule commands for later execution
- **batch** - Schedule commands for execution when system load permits

---

## Level 7: Communication & Job Control (Expert Level)

### 🏗️ Expert Level
- **write** - Send a message to another user
- **mesg** - Control write access to terminal
- **talk** - Talk to another user
- **fg** - Bring job to foreground
- **bg** - Put job in background  
- **jobs** - Display active jobs
- **wait** - Wait for process completion
- **alias** - Define or display aliases
- **command** - Execute a simple command
- **read** - Read from standard input

---

## Level 8: Development Tools (Expert Level)

### 🏗️ Expert Level
- **make** - Maintain program dependencies
- **ar** - Archive and library maintainer
- **nm** - Display symbol table
- **strip** - Remove symbols from object files
- **size** - Display section sizes
- **ranlib** - Generate archive index
- **what** - Identify SCCS keyword strings
- **admin** - Create and administer SCCS files
- **delta** - Make a delta to an SCCS file
- **get** - Get a version of an SCCS file
- **prs** - Print an SCCS file
- **rmdel** - Remove a delta from an SCCS file
- **sact** - Print current SCCS editing activity
- **sccs** - Front end for the SCCS subsystem
- **unget** - Undo a previous get of an SCCS file
- **val** - Validate SCCS files

---

## Level 9: Specialized & System-Specific (Expert Level)

### 🏗️ Expert Level
- **bc** - Arbitrary precision calculator language
- **dc** - Desk calculator
- **factor** - Factor integers
- **tabs** - Set terminal tabs
- **tput** - Initialize terminal or query terminfo database
- **tty** - Print terminal name
- **locale** - Display locale-specific information
- **localedef** - Define locale environment
- **iconv** - Convert character encoding
- **gencat** - Generate formatted message catalog
- **logger** - Make entries in the system log
- **mailx** - Send and receive mail
- **man** - Display manual pages
- **more** - File perusal filter for crt viewing
- **vi** - Screen-oriented text editor

---

## Recommended Implementation Order

Based on learning value and complexity progression:

1. **Start Here**: `true`, `false`, `echo`, `pwd` - Build confidence with simple utilities
2. **Text Basics**: `cat`, `head`, `tail`, `tr`, `cut` - Learn file I/O patterns  
3. **File Operations**: `cp`, `mv`, `rm`, `chmod` - Understand file system interactions
4. **Text Processing**: `sort`, `uniq`, `grep` - Algorithm implementation practice
5. **System Tools**: `ls`, `ps`, `find` - Complex system interactions
6. **Advanced**: `sed`, `awk`, `make` - Language processing and build tools

## Notes

- **mwc** is already implemented as a word count utility (equivalent to POSIX `wc`)
- Utilities marked as ⭐ are excellent next steps for learning
- Some utilities like `sh` (shell) are extremely complex and might be beyond the scope of this educational project
- Consider implementing utilities in the recommended order to build skills progressively
- Each utility should include comprehensive man page research and integration tests as per project guidelines

## References

- [POSIX.1-2024 Standard](https://pubs.opengroup.org/onlinepubs/9799919799/idx/utilities.html)
- [Linux man pages](https://man7.org/linux/man-pages/)
- Individual utility man pages: `man <utility_name>`