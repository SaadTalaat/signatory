//! Signatory Ed25519 provider for the [ed25519-dalek] crate.
//!
//! For a usage example, see the toplevel Signatory docs:
//! <https://docs.rs/signatory/latest/signatory/ed25519/index.html>
//!
//! [ed25519-dalek]: https://github.com/dalek-cryptography/ed25519-dalek

#![no_std]
#![deny(warnings, missing_docs, trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unused_import_braces, unused_qualifications)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tendermint/signatory/master/img/signatory-rustacean.png",
    html_root_url = "https://docs.rs/signatory-dalek/0.11.0"
)]

#[cfg(test)]
#[macro_use]
extern crate signatory;

use digest::Digest;
use ed25519_dalek::{Keypair, SecretKey};

use signatory::{
    ed25519,
    error::{Error, ErrorKind},
    PublicKeyed, Signature, Signer, Verifier,
};
use signatory::{generic_array::typenum::U64, DigestSigner, DigestVerifier};

/// Ed25519 signature provider for ed25519-dalek
pub struct Ed25519Signer(Keypair);

impl<'a> From<&'a ed25519::Seed> for Ed25519Signer {
    /// Create a new DalekSigner from an unexpanded seed value
    fn from(seed: &'a ed25519::Seed) -> Self {
        Ed25519Signer(keypair_from_seed(seed))
    }
}

impl PublicKeyed<ed25519::PublicKey> for Ed25519Signer {
    fn public_key(&self) -> Result<ed25519::PublicKey, Error> {
        Ok(ed25519::PublicKey::from_bytes(self.0.public.as_bytes()).unwrap())
    }
}

impl Signer<ed25519::Signature> for Ed25519Signer {
    fn sign(&self, msg: &[u8]) -> Result<ed25519::Signature, Error> {
        let signature = self.0.sign(msg).to_bytes();
        Ok(Signature::from_bytes(&signature[..]).unwrap())
    }
}

/// Ed25519ph (i.e. pre-hashed) signature provider for ed25519-dalek
pub struct Ed25519PhSigner(Keypair);

impl<'a> From<&'a ed25519::Seed> for Ed25519PhSigner {
    /// Create a new DalekSigner from an unexpanded seed value
    fn from(seed: &'a ed25519::Seed) -> Self {
        Ed25519PhSigner(keypair_from_seed(seed))
    }
}

impl PublicKeyed<ed25519::PublicKey> for Ed25519PhSigner {
    fn public_key(&self) -> Result<ed25519::PublicKey, Error> {
        Ok(ed25519::PublicKey::from_bytes(self.0.public.as_bytes()).unwrap())
    }
}

// TODO: tests!
impl<D> DigestSigner<D, ed25519::Signature> for Ed25519PhSigner
where
    D: Digest<OutputSize = U64> + Default,
{
    fn sign(&self, digest: D) -> Result<ed25519::Signature, Error> {
        // TODO: context support
        let context: Option<&'static [u8]> = None;

        let signature =
            Signature::from_bytes(&self.0.sign_prehashed(digest, context).to_bytes()[..]).unwrap();

        Ok(signature)
    }
}

/// Ed25519 verifier provider for ed25519-dalek
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ed25519Verifier(ed25519_dalek::PublicKey);

impl<'a> From<&'a ed25519::PublicKey> for Ed25519Verifier {
    fn from(public_key: &'a ed25519::PublicKey) -> Self {
        Ed25519Verifier(ed25519_dalek::PublicKey::from_bytes(public_key.as_ref()).unwrap())
    }
}

impl Verifier<ed25519::Signature> for Ed25519Verifier {
    fn verify(&self, msg: &[u8], sig: &ed25519::Signature) -> Result<(), Error> {
        let dalek_sig = ed25519_dalek::Signature::from_bytes(sig.as_ref()).unwrap();
        self.0
            .verify(msg, &dalek_sig)
            .map_err(|_| ErrorKind::SignatureInvalid.into())
    }
}

/// Ed25519ph (i.e. pre-hashed) verifier provider for ed25519-dalek
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ed25519PhVerifier(ed25519_dalek::PublicKey);

impl<'a> From<&'a ed25519::PublicKey> for Ed25519PhVerifier {
    fn from(public_key: &'a ed25519::PublicKey) -> Self {
        Ed25519PhVerifier(ed25519_dalek::PublicKey::from_bytes(public_key.as_ref()).unwrap())
    }
}

// TODO: tests!
impl<D> DigestVerifier<D, ed25519::Signature> for Ed25519PhVerifier
where
    D: Digest<OutputSize = U64> + Default,
{
    fn verify(&self, digest: D, sig: &ed25519::Signature) -> Result<(), Error> {
        // TODO: context support
        let context: Option<&'static [u8]> = None;
        let dalek_sig = ed25519_dalek::Signature::from_bytes(sig.as_ref()).unwrap();
        self.0
            .verify_prehashed(digest, context, &dalek_sig)
            .map_err(|_| ErrorKind::SignatureInvalid.into())
    }
}

/// Convert a Signatory seed into a Dalek keypair
fn keypair_from_seed(seed: &ed25519::Seed) -> Keypair {
    let secret = SecretKey::from_bytes(seed.as_secret_slice()).unwrap();
    let public = ed25519_dalek::PublicKey::from(&secret);
    Keypair { secret, public }
}

#[cfg(test)]
mod tests {
    use super::{Ed25519Signer, Ed25519Verifier};
    ed25519_tests!(Ed25519Signer, Ed25519Verifier);
}
