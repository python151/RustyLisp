# LAUGH Interpreter

This is a LISP-based language that I'm making following the UoE <a href="https://typesig.pl/resources/lisp-workshop">TypeSig workshops</a>. 

The language itself is designed to be esoteric and comedic, while implementing helpful error messages and checks. For instance, the standard file extension is .lmao_why_are_you_using_this, and the interpretter will helpfully (albiet not so nicely) inform you of this if one makes a typo.

## Goals

My main goals of the project are as follows:
<ol>
  <li>Successfully implement a Turing complete language with a sufficiently powerful structure to perform basic calculations and operations, as well as implement simple data structures.</li>
  <li>Learn about programming language theory and the inner workings of programming languages.</li>
  <li>Have fun developing an esoteric language!</li>
</ol>

## Basic syntax and examples

Adding two integers:
<br>
`(add 1 2) ; produces 3` 
<br>
Working with strings:
<br>
`concat "Hello " (concat "world" "!") ; produces "Hello world!"`
<br>
Basic division:
<br>
`divide 3 4 ; produces .75`
