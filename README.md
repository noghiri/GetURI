# GetURI
GetURI is a tool I created to fill a need. An organization I've worked for needed a simple tool in the drop-down right click menu to create a link to server resources on the SMB server for use with their intranet site.
I needed to learn a new programming language.
Thus, this project was born.

## Installation: The Easy Way
Just head over to the latest release, and run the installer.  Releases are found [here.](https://github.com/noghiri/GetURI/releases)

## Installation: The Hard Way
You'll need to compile the binary, put it somewhere, and then reference it via the registry.
To add it to the context menu, I currently use these keys:

    HKCR\*\shell\GetURI with value "Get &URI"
    HKCR\*\shell\GetURI\command with value """<path_to_file>\geturi.exe"" ""%1"""
Basically, this adds a GetURI key to the shell, and ties it to a command that passes the file as Environment Argument 1 to the program, wrapped in quotes so the program is happy.
