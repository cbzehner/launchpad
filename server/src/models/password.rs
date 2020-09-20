use rocket::http::RawStr;
use rocket::request::FromFormValue;
use scrypt::{scrypt_check, scrypt_simple, ScryptParams};

#[derive(Copy, Clone)]
enum Mode {
    Raw,
    Digest,
}

/// Securely store a password in a format where the raw value of the password is not acessible.
#[derive(Clone)]
pub struct Password {
    password: String,
    mode: Mode,
}

impl Password {
    pub fn new(password: &str) -> Self {
        // TODO (security): Reject weak passwords with https://github.com/magiclen/passwords or https://github.com/shssoichiro/zxcvbn-rs or Django's 2k password list
        Password {
            password: password.into(),
            mode: Mode::Raw,
        }
    }

    /// Encrypt the password using scrypt with a random salt and encoding the algorithm and salt in the returned string.
    /// The first call to `digest()` will consume the raw password value, replacing it with the derived key. Subsequent
    /// calls will simply return a `clone()` of the derived key.
    pub fn digest(&mut self) -> String {
        match self.mode {
            Mode::Raw => self.securely_hash(),
            Mode::Digest => self.password.clone(),
        }
    }

    /// Verify a digest against the raw password value.
    /// Does not work if the raw password has already been used to generate a new digest.
    pub fn verify_digest(
        &mut self,
        candidate_digest: &str,
    ) -> Result<(), scrypt::errors::CheckError> {
        match self.mode {
            Mode::Raw => scrypt_check(&self.password, candidate_digest),
            Mode::Digest => Err(scrypt::errors::CheckError::InvalidFormat),
        }
    }

    /// Run the work of a password check with no useful result. Useful for defeating timing attacks and masking
    /// code paths that conditionally verify an inputted password. For example, the existing of a user account.
    pub fn do_work() -> () {
        // A salted scrypt hash of the string "password".
        let random_digest =
"$rscrypt$0$DwgB$MaI1KADL1gF/PtG55VBnpw==$oZJvNPwYLwMFfIz7FX3oc4nx+JLDDtd9w7LX/xr20/g=$";
        let _ = scrypt_check("decoypassword", random_digest);
        ()
    }

    // TODO (performance): Consider switching to Argon2 or allowing it as an option.
    /// Calculate the hash of the raw password. If the initial raw value is still set, hash the password and replace the raw value with the hashed value.
    fn securely_hash(&mut self) -> String {
        let params = ScryptParams::recommended();
        let digest = scrypt_simple(&self.password, &params).expect("OS RNG should not fail");

        self.password = digest.clone();
        self.mode = Mode::Digest;

        digest
    }
}

impl<'a> FromFormValue<'a> for Password {
    type Error = &'static str;

    fn from_form_value(v: &'a RawStr) -> Result<Self, Self::Error> {
        // TODO (security): Validate password strength (see examples/form_validation)
        Ok(Password::new(v))
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Password {{..}}")
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Password {{..}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    /// Even in cases where the same password is used, they should result in different derived keys (a.k.a. digests).
    fn unique_hash_per_instance() {
        let mut password_one = Password::new("password");
        let mut password_two = Password::new("password");

        assert_ne!(password_one.digest(), password_two.digest());
    }

    #[test]
    /// Subsequent calls to digest() on the same instance of the struct should return the same result.
    fn digest() {
        let mut password = Password::new("password");

        let digest_one = password.digest();
        let digest_two = password.digest();

        assert_eq!(digest_one, digest_two);
    }

    #[test]
    /// Confirm whether or not a raw password matches a digest
    fn verify_digest() {
        let password_digest = "$rscrypt$0$DwgB$wzJHTVlpeoATfN3TZ6i+dw==$wfkXr6V1pz0W/dO0cycnYNkEwVUGKYAc0UPyTBXA8+U=$";
        let decoypassword_digest = "$rscrypt$0$DwgB$MaI1KADL1gF/PtG55VBnpw==$oZJvNPwYLwMFfIz7FX3oc4nx+JLDDtd9w7LX/xr20/g=$";
        let mut password = Password::new("password");

        assert_eq!(password.verify_digest(&password_digest), Ok(()));
        assert_eq!(
            password.verify_digest(&decoypassword_digest),
            Err(scrypt::errors::CheckError::HashMismatch)
        );
    }

    #[test]
    /// Debug-mode viewing should not leak internal data
    fn opaque_debug() {
        let mut password = Password::new("password");
        assert_eq!(format!("{:?}", password), "Password {..}"); // Mode::Raw
        password.digest();
        assert_eq!(format!("{:?}", password), "Password {..}"); // Mode::Digest
    }

    #[test]
    /// Display-mode viewing should not leak internal data
    fn opaque_display() {
        let mut password = Password {
            password: "password".into(),
            mode: Mode::Raw,
        };
        assert_eq!(format!("{}", password), "Password {..}"); // Mode::Raw
        password.digest();
        assert_eq!(format!("{}", password), "Password {..}"); // Mode::Digest
    }
}
