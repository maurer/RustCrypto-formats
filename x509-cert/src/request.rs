//! PKCS#10 Certification Request types

use crate::ext::Extension;
use crate::{attr::Attributes, name::Name};

use alloc::vec::Vec;

use const_oid::db::rfc5912::ID_EXTENSION_REQ;
use const_oid::{AssociatedOid, ObjectIdentifier};
use der::asn1::BitString;
use der::{Decode, Enumerated, Sequence};
use spki::{AlgorithmIdentifierOwned, SubjectPublicKeyInfoOwned};

#[cfg(feature = "pem")]
use der::pem::PemLabel;

/// Version identifier for certification request information.
///
/// (RFC 2986 designates `0` as the only valid version)
#[derive(Clone, Debug, Copy, PartialEq, Eq, Enumerated)]
#[asn1(type = "INTEGER")]
#[repr(u8)]
pub enum Version {
    /// Denotes PKCS#8 v1
    V1 = 0,
}

/// PKCS#10 `CertificationRequestInfo` as defined in [RFC 2986 Section 4].
///
/// ```text
/// CertificationRequestInfo ::= SEQUENCE {
///     version       INTEGER { v1(0) } (v1,...),
///     subject       Name,
///     subjectPKInfo SubjectPublicKeyInfo{{ PKInfoAlgorithms }},
///     attributes    [0] Attributes{{ CRIAttributes }}
/// }
/// ```
///
/// [RFC 2986 Section 4]: https://datatracker.ietf.org/doc/html/rfc2986#section-4
#[derive(Clone, Debug, PartialEq, Eq, Sequence)]
pub struct CertReqInfo {
    /// Certification request version.
    pub version: Version,

    /// Subject name.
    pub subject: Name,

    /// Subject public key info.
    pub public_key: SubjectPublicKeyInfoOwned,

    /// Request attributes.
    #[asn1(context_specific = "0", tag_mode = "IMPLICIT")]
    pub attributes: Attributes,
}

/// PKCS#10 `CertificationRequest` as defined in [RFC 2986 Section 4].
///
/// ```text
/// CertificationRequest ::= SEQUENCE {
///     certificationRequestInfo CertificationRequestInfo,
///     signatureAlgorithm AlgorithmIdentifier{{ SignatureAlgorithms }},
///     signature          BIT STRING
/// }
/// ```
///
/// [RFC 2986 Section 4]: https://datatracker.ietf.org/doc/html/rfc2986#section-4
#[derive(Clone, Debug, PartialEq, Eq, Sequence)]
pub struct CertReq {
    /// Certification request information.
    pub info: CertReqInfo,

    /// Signature algorithm identifier.
    pub algorithm: AlgorithmIdentifierOwned,

    /// Signature.
    pub signature: BitString,
}

#[cfg(feature = "pem")]
impl PemLabel for CertReq {
    const PEM_LABEL: &'static str = "CERTIFICATE REQUEST";
}

impl<'a> TryFrom<&'a [u8]> for CertReq {
    type Error = der::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        Self::from_der(bytes)
    }
}

/// `ExtensionReq` as defined in [RFC 5272 Section 3.1].
///
/// ```text
/// ExtensionReq ::= SEQUENCE SIZE (1..MAX) OF Extension
/// ```
///
/// [RFC 5272 Section 3.1]: https://datatracker.ietf.org/doc/html/rfc5272#section-3.1
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtensionReq(pub Vec<Extension>);

impl AssociatedOid for ExtensionReq {
    const OID: ObjectIdentifier = ID_EXTENSION_REQ;
}

impl_newtype!(ExtensionReq, Vec<Extension>);
