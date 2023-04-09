# resume-builder

Run with `cargo run`

Program will output:

```json
{
    "about": {
        "email": "testsdsad@asdddas.com",
        "phone": "+47 12345678",
    },
}
```

even if cv.ini is full of whitespaces.

Format .pest files:

```
cargo install pest_fmt
pestfmt .
```