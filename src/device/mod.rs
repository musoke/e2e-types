use core::fmt;
use core::convert::TryFrom;
use std::ascii::AsciiExt;
use rand::thread_rng;
// use serde::{Deserialize, Serialize, Serializer};

#[derive(Fail, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    #[fail(display = "invalid characters in DeviceId")] InvalidCharacters,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(
    // TODO: Any requirements on format? Spec just says string; most examples seem to be ~10 upper
    // case letters
    String,
);

impl DeviceId {
    pub fn new() -> DeviceId {
        use rand::Rng;

        // TODO: Should the device_id be cryptographically random?
        let device_id: String = thread_rng()
            .gen_ascii_chars()
            .filter(|&c| c.is_ascii_uppercase())
            .take(10)
            .collect();

        DeviceId::try_from(&device_id[..]).expect("we just generated it")
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> TryFrom<&'a str> for DeviceId {
    type Error = Error;

    // TODO: clarify spec's requirements on device_id
    // All examples I've seen have been uppercase ascii, but this isn't said explicitly, and
    // presumably there is some length limit.
    fn try_from(s: &'a str) -> Result<DeviceId, Error> {
        if s.is_ascii_uppercase() {
            Ok(DeviceId(s.into()))
        } else {
            Err(Error::InvalidCharacters)
        }
    }
}

#[cfg(test)]
mod test {
    use super::DeviceId;
    use core::convert::TryFrom;
    use std::ascii::AsciiExt;

    #[test]
    fn from_invalid_string() {
        let device_id = DeviceId::try_from("abc.");

        assert!(device_id.is_err());
    }

    #[test]
    fn generate() {
        let device_id = DeviceId::new();

        assert!(device_id.0.is_ascii_uppercase());
    }
}

#[cfg(test)]
mod test_serde {
    use core::convert::TryFrom;
    use super::DeviceId;
    use serde_json;
    // use serde_test::{assert_tokens, Token};

    #[test]
    fn serialize_valid() {
        let d = "AAA";
        let device_id = DeviceId::try_from(d).expect("valid device ID");

        // assert_tokens(&device_id, &[Token::Str(&d[..])]);
        assert_eq!(
            serde_json::to_string(&device_id).expect("Failed to convert device_id to JSON"),
            format!(r#""{}""#, d)
        )
    }

    #[test]
    fn deserialize_valid() {
        let d = "AAA";
        let s = format!(r#""{}""#, d);
        let device_id = DeviceId::try_from(d).expect("valid device ID");

        // assert_tokens(&device_id, &[Token::Str(&d[..])]);
        assert_eq!(
            serde_json::from_str::<DeviceId>(&s).expect("Failed to convert device_id to JSON"),
            device_id
        )
    }
}
