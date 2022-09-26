Demonstration of creating a Flux runtime that can be usable from a
compiled or interpretered language.

Requires Rust, Go, and a compiler (clang used in the example).

Use the following commands to compile:

```
$ cargo build
$ go build -buildmode=c-archive ./vmrt
$ clang -Ivmrt script.c target/debug/libflux_vm.a vmrt.a -framework CoreFoundation
```

On Linux, remove `-framework CoreFoundation`. I am not sure if other
libraries need to be linked.

You can then execute the script with `./a.out`. You can modify the
script as much as you want and only rerun the `clang` compilation to
change the script. Once the Rust and Go are built, they can be reused
over and over again without changes.
