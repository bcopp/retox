# Retox

Retox is a command line utility that allows linux users to rename multiple files in one pass. 

The most notable feature within Retox is its option to specify multiple key/value replacement pairs. The program passes through each targeted file replacing each matching key string with a value string. Retox uses a simple priority to determine its replacement behavior. 

## Prerequisities

Before you begin, ensure you have met the following requirements:
* You have **Rust** installed on your system.
* You have a **Linux** machine.

## Installing Retox

To install Retox, enter the following line into your terminal:

```
cargo install retox
```

## Quick Start

The DEFAULT behavior of Retox is to run in "dry mode". This mode will print the expected results to the screen but make no changes. The option `-r` or `--run` must be specified if you would like the changes to take effect. 

A basic Retox command

```
retox -f filename_is_foo.txt -s 'name=,foo=foobar'
RESULT:  file_is_foobar.txt
```
* -f : Specifies file name/location 
* -s : Specifies that every 'name' sequence is replaces with ''
       and that every 'foo' sequence is replaced with 'foobar'
       

## Using Retox

If you would like to inspect retox's options in the terminal please type.
`retox --help` `retox --explain` or `man retox`

USAGE:

    retox [FLAGS] [OPTIONS]

FLAGS:

        --explain    Prints a more verbose help section

    -F               Will cause the program to NOT throw an error if a specified directory is invalid

    -h, --help       Prints help information

    -q               Does not print anything to stdout

    -R               Searches the specified directory recusively

    -r, --run        Writes the modified names to the file system

    -V, --version    Prints version information

    -v               Specifies the verbosity of the program

OPTIONS:

    -d, --dir <DIRS>...    Sets the directory to work within. (If root is not specified is assumes a relative path)

    -f <FILES>...          Specifies a specific file to rename. (If root is not specified is assumes a relative path)

    -s, --seq <SEQ>        Defines the characters to replace for each filename


        
EXAMPLES

To explore how Retox prioritizes overlapping replacements click here: [Replacement Priority](https://github.com/bcopp/rreplace)
```
--------------------------------------------------

        retox -f filename_is_foo.txt -s 'name=,foo=foobar'
        RESULT:  file_is_foobar.txt

        retox -f filename_is_foo.txt -s 'filename=xxx,name=yyy'
        RESULT:  xxx_is_foobar.txt
        
        retox -f filename_is_foo.txt -s 'filename=xxx,filename_is=yyy'
        RESULT:  yyy_foobar.txt
        
        retox -f filename_is_foo.txt -s 'i=I,filename_is_yzx=yyy'
        RESULT:  fIlename_Is_foo.txt
--------------------------------------------------
```

## Contributing to Retox
To contribute to Retox, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`. 
3. Make your changes and commit them: `git commit -m '<commit_message>'
4. Push to the original branch: `git push origin <project_name>/<location>`
5. Create the pull request.

Alternatively see the GitHub documentation on [creating a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

## Contact 

If you want to contact me you can reach me at bcopp.oss@gmail.com

## License 

This project uses the following license: [MIT](https://opensource.org/licenses/MIT).

