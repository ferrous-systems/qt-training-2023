# Library

In this tutorial you'll get familiar with:

* Refactoring your Rust project into multiple libraries (crates)
* Running applications on the command-line
* Parsing command line paramenters by hand
* Re-using existing crates in your application
* Learn about borrowing

We start with a command-line tool that takes an image and a filter name as input.
It applies the given filter to the image and produces an `output.png`.
