# JimTeX

## Description

A short summary of what JimTeX currently is: \
    - A turing-complete programming language with syntax inspired by LaTeX. \
    - A kind of sequel to my compiler, this is a fully interpreted language, intended to be embedded into the JimTeX IDE. \
    - Partially functional (functions can be passed as functions to functions, but not returned, mutability is handled with reassignment) \
    - Examples are in the jimtex_interpreter library as tests, run with `$ cargo test -- --nocapture` to view console output. 

## Goals

More than just a programming language intended to allow mathematicans to translate their 
LaTeX to a program (involving transpilers for several popular languages, such as SAGE/Python, Mathematica, Haskell, OCaml etc),
it would allow users to verify their proofs using formal methods similar to Coq using translation to lambda calculus and the Curry-Howard Isomorphism.

## Current Progress

- [x] Interpreter working (v0.1.0)
- [ ] IDE with Interpreter integration 
- [ ] Translation to other languages 
- [ ] Formal Verification of proofs 
