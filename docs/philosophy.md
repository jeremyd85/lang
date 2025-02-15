# Mission

## Flexibility

The language should be flexible for different language concepts. You should be able to have anywhere between void pointers in C or Go-like garbage collection without leaving the language ecosystem. You may have to make a core library or compiler extension to support such things, but no changes to the compiler should be required.

## Portability

You should be able to run the language on every platform with a reasonable lift. Embedded development should be kept in mind. It shouldn't be too hard to add a new target with a subset of functionality.

> *Example:*
For instance maybe small embedded systems would not support runtime GC.

## Make "bad" things hard

You should be encouraged to program well based on what is hard to do and what is easy to do. The idea is that even bad things may sometimes be the best option if for performance or creating powerful abstractions. Those things should still be done, but the programmer shouldn't want to do it since it is cool.

> *Example:*
It should be hard to overload operators for types.
