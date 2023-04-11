# resume-builder

Run with `cargo run`

Program will output:

```rust
CVData {
    about: About {
        phone: "+47 12345678",
        email: "testsdsad@asdddas.com",
    },
}
```

even if cv.ini is full of whitespaces.

Format .pest files:

```
cargo install pest_fmt
pestfmt .
```