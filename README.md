# RustShell

POSIX compliant shell that's capable of
interpreting shell commands, running external programs and own builtin commands like
cd, pwd, echo and more

## How to start

Cargo is required to build the project. If you don't have it installed, you can install it from [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## How to run

```sh
./your_program.sh
```

and you should see the prompt `$ `

```shell
$ ls
Cargo.lock  Cargo.toml  README.md  src  target

$ echo "Hello,     World!"
Hello,     World!

$ pwd
/home/username/path/to/your/project

$ cd ~
/home/username
```