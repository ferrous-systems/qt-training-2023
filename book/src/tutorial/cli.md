# Command-line interface

In this tutorial you'll get familiar with:

* Building Rust code for your local target
* Running applications on the command-line
* Parsing command line parameters by hand
* Re-using existing crates in your application
* Rust type systems basics

We start with a command-line tool that takes an image and a filter name as input.
It applies the given filter to the image and produces an `output.png`.
