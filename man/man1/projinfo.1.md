% PROJINFO(1) 0.1.0 | ProjInfo Documentation
% dk949
% 15 March 2022

# NAME

projinfo - Language stats for your projects

# SYNOPSIS

projinfo \[OPTIONS] \[DIR]

# DESCRIPTION

Display distribution of languages used in a project. What language a file
belongs to is deduced from the file extension.


# CONFIGURATION

You can change which language maps to which file extension by editing the
`langs.json` file located in `your/config/directory/projinfo/`.

The config directory is:

On Linux $XDG_CONFIG_HOME or $HOME/.config

On MacOs $HOME/Library/Preferences

# OPTIONS

**-a**, **--all**
: Include all file types

**-h**, **--help**
: Print help information

**-i**, **--ignore** <IGNORE>
: List of files or directories to ignore You can pass list as a single
 comma-separated list, or by using the flag multiple times.

**-m**, **--most** <MOST>
: Maximum number of entries to show. (default: 5)

**--no-git**
: This flag is a placeholder for future functionality. Do not use git even if
 the directory's a git repository.

**--no-skip-dots**
: Do not skip hidden/dot directories

**-t**, **--types** <TYPES>
: List of file types to include in the summary. Does not support passing as
 comma-separated list. (possible values: programming, markup, data, prose)

**-V**, **--version**
: Print version information

# BUGS

No known bugs. To report any [do so on GitHub](https://github.com/dk949/ProjInfo/issues).
