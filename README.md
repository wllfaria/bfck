# Bfck

Well, i guess what the world was needing was yet another brainfuck interpreter
or compiler.

## What?

This is a simple brainfuck compiler and interpreter written entirely in rust that
has a very basic repl attached to it.

## Why?

Not sure. Lets say I was really bored and decided this was a good idea.

## Features

### CLI

bfck has a built-in cli that is how you interface with the compiler.

```
Interpreter and compiler for brainfuck

Usage: bfck [OPTIONS] [source] [output]

Arguments:
  [source]  Source file to compile
  [output]  Output file name

Options:
  -s, --assemble  Generate the assembly output of the Brainfuck code
  -h, --help      Print help
  -V, --version   Print version

When no options are passed, a REPL is initialized instead
```

## Why should you care?

You should not, like really, I don't know why I did this. But since you're here,
have fun.
