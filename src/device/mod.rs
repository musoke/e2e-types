use core::fmt;
use rand::thread_rng;

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
        let device_id = thread_rng().gen_ascii_chars().take(10).collect();
        DeviceId { id: device_id }
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<S> From<S> for DeviceId
where
    S: Into<String>,
{
    /// # Examples
    ///
    /// ```
    /// let d = olm::device::DeviceId::from("DEVID");
    /// let s: String = d.to_string();
    /// ```
    fn from(s: S) -> DeviceId {
        DeviceId { id: s.into() }
    }
}

// TODO: Rewrite with std::convert
impl DeviceId {
    pub fn to_string(&self) -> String {
        self.id.clone()
    }
}

#[cfg(test)]
mod test {}
