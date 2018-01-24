use core::fmt;
use core::convert::TryFrom;
use std::ascii::AsciiExt;
use rand::thread_rng;

#[derive(Fail, Debug, Copy, Clone, PartialEq)]
pub enum Error {
    #[fail(display = "invalid characters in DeviceId")] InvalidCharacters,
}

#[derive(Debug, Clone)]
pub struct DeviceId {
    // TODO: Any requirements on format? Spec just says string; most examples seem to be ~10 upper
    // case letters
    id: String,
}

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
        write!(f, "{}", self.id)
    }
}

impl<'a> TryFrom<&'a str> for DeviceId {
    type Error = Error;

    // TODO: clarify spec's requirements on device_id
    // All examples I've seen have been uppercase ascii, but this isn't said explicitly, and
    // presumably there is some length limit.
    fn try_from(s: &'a str) -> Result<DeviceId, Error> {
        if s.is_ascii_uppercase() {
            Ok(DeviceId { id: s.into() })
        } else {
            Err(Error::InvalidCharacters)
        }
    }
}

// TODO: Rewrite with std::convert
impl DeviceId {
    pub fn to_string(&self) -> String {
        self.id.clone()
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

        assert!(device_id.id.is_ascii_uppercase());
    }
}
