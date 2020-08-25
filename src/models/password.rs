use rocket::http::RawStr;
use rocket::request::FromFormValue;
use scrypt::{scrypt_check, scrypt_simple, ScryptParams};

#[derive(Copy, Clone)]
pub struct Password<'v>(&'v str);

impl<'a> Password<'a> {
    pub fn digest(&mut self) -> Option<String> {
        let raw_password = self.0;

        if raw_password.is_empty() {
            return None;
        }

        // TODO: Investigate whether there is a clever way to implement this.
        // Allow a password to be read once-and-only-once by replacing it with an empty string.
        self.0 = "";
        Some(self.securely_hash(raw_password))
    }

    fn securely_hash(self, password: &str) -> String {
        let params = ScryptParams::recommended();
        // TODO (upgrade): post-release of rocket 0.5, provide the ROCKET_SECRET as a salt to scrypt.
        scrypt_simple(password, &params).expect("OS RNG should not fail")
    }

    pub fn verify_digest(self, password_digest: &str) -> Option<()> {
        scrypt_check(self.0, password_digest).ok()
    }
}

impl<'v> FromFormValue<'v> for Password<'v> {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        // TODO: Validate password strength (see examples/form_validation)
        // Reject weak passwords with https://github.com/magiclen/passwords or https://github.com/shssoichiro/zxcvbn-rs
        Ok(Password(v.as_str()))
    }
}

impl<'v> std::fmt::Debug for Password<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Password {{..}}")
    }
}

impl<'v> std::fmt::Display for Password<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Password {{..}}")
    }
}

impl<'v> std::ops::Deref for Password<'v> {
    type Target = &'v str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'v> std::ops::DerefMut for Password<'v> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn opaque_debug() {
        let password = Password("NeverLeakAPassword");
        assert_eq!(format!("{:?}", password), "Password {..}");
    }

    #[test]
    fn opaque_display() {
        let password = Password("NeverLeakAPassword");
        assert_eq!(format!("{}", password), "Password {..}");
    }

    #[test]
    fn digest_once() {
        let mut password = Password("NeverLeakAPassword");
        assert_eq!(password.digest().is_some(), true);
        assert_eq!(password.digest().is_none(), true);
    }
}
