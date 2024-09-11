# Icyshell 🛠️ with 🧊
a new shell for experiments.
> [!WARNING]
> you should not use this shell because of it is not useful and good quality.
## How to use
If you want to use this shell, you have to build from source.  
 [!TIP]
> If you want to set some settings, you have to make `~/.icyshrc`.
## Settings
### start_with/st
If you want to output some message, you can use `start_with`/`st` like the following:
```
start_with = '''
some messages!!
'''
```
> [!WARNING]
> you cannot remove the space. If you remove them, the shell happen errors.
### comments
If you want to add some comments, you have to add `//` on beginning of line.
```
//comment!
```
> [!NOTE]
> However, you can't use comment in triple quotes block(`''' '''`).
> ```
> //The following code treats comment(`//not comment`) as a string.
> start_with = '''
> //not comment
> '''
> ```
### code/cd
If you want to execute some commands, you can use `code`/`cd`.
```
code = one command
```
Do you want to execute some commands at once? you can use triple quotes block(`''' '''`).
```
code = '''
command1
command2
'''
```
### var
do you think that you want to retain the command output? `var` can retain **a** command.
```
//The following code will retain output of `command1`.
var =
code = command1
```
> [!WARNING]
> If you weren't able to retain multiple outputs, it's correct.
> The var cannot retain multiple outputs.
### prefix
do you want to change the shell prefix?(for example, $, % and @)
```
prefix = $
```
also you can use a command output.
```
prefix = var
```
or
```
prefix code("a command")
```
