use crate::api::CspTlsHandshakeSignerProvider;
use crate::secret_key_store::SecretKeyStore;
use crate::{Csp, TlsHandshakeCspServer};
use rand::{CryptoRng, Rng};
use std::sync::Arc;

/// Note that `R: 'static` and `S: 'static` here does not mean that `R` and `S`
/// must _have_ a 'static lifetime, but rather that `R` and `S` are _bounded by_
/// a 'static lifetime. Also note that `&'static T` and `T: 'static` are _not_
/// the same thing, and that `T: 'static` means that `T` can be a borrowed type
/// with a 'static lifetime or an _owned_ type.
///
/// Said differently, the 'static trait bound on a generic type T requires that
/// any references inside the type must live at least as long as 'static. For
/// our purposes where `R` and `S` are (at least currently) always owned types
/// (i.e. don't have references) in the form of `OsRng`/`ChaCha20Rng` for `R`
/// and `ProtoSecretKeyStore`/`VolatileSecretKeyStore` for `S`, the 'static
/// lifetime bounds on `R` and `S` have no impact at all.
///
/// See also [Common Rust Lifetime Misconceptions].
///
/// [Common Rust Lifetime Misconceptions]: https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program
impl<R: Rng + CryptoRng + Send + Sync + 'static, S: SecretKeyStore + 'static>
    CspTlsHandshakeSignerProvider for Csp<R, S>
{
    fn handshake_signer(&self) -> Arc<dyn TlsHandshakeCspServer> {
        Arc::clone(&self.csp_server) as Arc<_>
    }
}
