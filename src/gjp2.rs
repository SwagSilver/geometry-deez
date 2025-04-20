use crate::user::Password;
use bcrypt::{self, DEFAULT_COST, BcryptError};
use sha1::{Sha1, Digest};

const SUFFIX: &'static str = "mI29fmAnxgTs";

pub struct Gjp2(String);

impl Gjp2 {
    fn new(gjp2: &str) -> Self {
        Self(gjp2.to_owned())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub struct Gjp2Error(pub BcryptError);

impl From<BcryptError> for Gjp2Error {
    fn from(error: BcryptError) -> Self {
        Self(error)
    }
}

fn verify_gjp2(text: &str, gjp2: Gjp2) -> Result<bool, Gjp2Error> {
    Ok(bcrypt::verify(text, gjp2.as_str())?)
}

pub struct Gjp2Generator {
    digest: Sha1,
}

impl Gjp2Generator {
    pub fn new(digest: Sha1) -> Self {
        Self {
            digest,
        }
    }

    pub fn generate_gjp2(
        &mut self,
        password: Password
    ) -> Result<Gjp2, Gjp2Error> {
        self.digest.update(password.as_str().to_owned() + SUFFIX);
        let sha1_hash = self.digest.clone().finalize();
        self.digest.reset();

        let bcrypt_hash = bcrypt::hash(sha1_hash, DEFAULT_COST)?;
        Ok(Gjp2::new(bcrypt_hash.as_str()))
    }
}
