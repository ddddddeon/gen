# gen
Generate project directory structure and boilerplate

## Installation
```sh
git clone https://github.com/ddddddeon/gen
cd gen
make release
make install
```

## Usage
`gen` can generate boilerplate for C, C++, Rust and Java projects. 

- The first positional argument is the language
- The second positional argument is the project name
- The third positional argument is the project type-- `bin`/`binary` or `lib`/`library`
- The `--domain` flag is used for Java projects

Templates are stored in `templates` in the root of this repository.

```sh
gen c foobar # generate a new C project named foobar
gen c foobar lib # generate a new C library named foobar
gen cpp foobar # generate a new C++ project named foobar
gen cpp foobar lib # generate a new C++ library named foobar
gen rust foobar # generate a new Rust project named foobar
gen rust foobar lib # generate a new Rust library named foobar
gen java foobar --domain com.ddddddeon # generate a new Java project named foobar with domain com.ddddddeon
```
