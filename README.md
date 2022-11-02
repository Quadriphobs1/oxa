# oxa

Yet another rust lox interpreter

This is a rust code implementation for lox interpreter from the book [Crafting interpreter](https://craftinginterpreters.com),
the reference book above originally implemented lox in Java programming language which support OOP, since rust isn't fully OOP
the object representation is baked into [oxa](./oxa) package (see [object.rs](./oxa/src/object.rs)).

There is has been many implementation example of the lox language in many programming language, this is yet another one :smile


> This is strictly for learning purpose to help understand how compiler and interpreter work.

## Features

List of supported features

- Parsing
- Scanning
- Interpreting

## Syntax

// TODO: Write up the syntax format for oxa language.

## Packages

- [ast_generator](./ast_generator): Utility code to generate boilerplate code files for expressions, statement e.t.c.
- [oxa](./oxa): The oxa language interpreter code.



## License

Licensed under [MIT](http://opensource.org/licenses/MIT) license

## Acknowledgement

Kudos to [Robert Nystrom](https://twitter.com/intent/user?screen_name=munificentbob) for an awesome written text. It has been such an enjoyable journey following through the thought process.
