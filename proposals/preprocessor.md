# Proposal for a new preprocessor for the language

## The preprocessor is based on the following principles:

1. The preprocessor is a separate program that is run before the lexer.
2. The preprocessor is a simple text processor that does not understand the language.

## Comments

Comments are removed from the source code before the lexer is run. Comments are defined as follows:

1. A comment starts with `//` and ends at the end of the line.
2. A comment starts with `/*` and ends with `*/`.

## Macros

Macros are defined as follows:

1. A macro starts with `#define X Y` and ends at the end of the line.

## Imports

Imports are defined as follows:

1. An import starts with `#import X as Y` and ends at the end of the line.
