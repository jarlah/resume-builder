#[derive(Debug, Default, PartialEq)]
pub struct About {
    pub phone: String,
    pub email: String,
}

#[derive(Debug, Default)]
pub struct CVData {
    pub about: About,
}

fn parse_property(input: &str) -> (&str, &str) {
    let (_, input) = input.split_once('=').unwrap_or((input, ""));
    let (value, input) = input.split_once('\n').unwrap_or((input, ""));
    (value.trim(), input)
}

pub fn parse_about(input: &str) -> (About, &str) {
    let mut about: About = Default::default();
    let (email, input) = parse_property(input);
    about.email = email.to_string();
    let (phone, input) = parse_property(input);
    about.phone = phone.to_string();
    (about, input)
}

#[test]
fn test_parse_about() {
    assert_eq!(
        parse_about(
            "[about]
email=test@email.com
phone=+4712345678
"
        ),
        (
            About {
                phone: "+4712345678".to_string(),
                email: "test@email.com".to_string()
            },
            ""
        )
    );
}
