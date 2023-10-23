# Py-Ru Technik demo

Demo repository to show the setup of a Rust-Python code.

## Resources

**Docker images:**

- https://hub.docker.com/_/rust
- https://hub.docker.com/_/python

**Rust Strings and memory:**

- https://doc.rust-lang.org/std/boxed/struct.Box.html
- https://doc.rust-lang.org/nightly/std/ffi/struct.CString.html
- https://doc.rust-lang.org/nightly/std/ffi/struct.CStr.html

**Rust C external API:**

- https://docs.rust-embedded.org/book/interoperability/c-with-rust.html
- https://docs.rust-embedded.org/book/interoperability/rust-with-c.html

**Cython Strings and Typed Memory Views:**

- https://cython.readthedocs.io/en/latest/src/tutorial/strings.html
- https://cython.readthedocs.io/en/latest/src/userguide/memoryviews.html

**Cython classes and Python constructs:**

- https://cython.readthedocs.io/en/latest/src/tutorial/cdef_classes.html
- https://docs.python.org/3/reference/datamodel.html#context-managers
- https://peps.python.org/pep-0544/

**Python setup config:**

- https://cython.readthedocs.io/en/latest/src/userguide/source_files_and_compilation.html

## Further work

The configuration of the project is not optimal. In particular,
`setup.py` is not properly written and we miss some `Makefile`
to enhance building.

The Rust structure `Example` should have been expressed by a
Python class. This is something that is left to do.
(The class should be expressed in Python by a `Protocol`, while
implementing a Context Manager for safe memory management.)
