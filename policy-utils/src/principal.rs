//! Principals
//!
//! Types and definitions in this module are used to describe Veracruz
//! principals, namely their identities and capabilities.
//!
//! ## Authors
//!
//! The Veracruz Development Team.
//!
//! ## Licensing and copyright notice
//!
//! See the `LICENSE_MIT.markdown` file in the Veracruz root directory for
//! information on licensing and copyright.

use super::error::PolicyError;
use crate::{parsers::parse_pipeline, pipeline::Expr};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, path::PathBuf, string::String, vec::Vec};
use wasi_types::Rights;

////////////////////////////////////////////////////////////////////////////////
// File operation and capabilities.
////////////////////////////////////////////////////////////////////////////////

/// The Principal of Capability in Veracruz.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Principal {
    /// The Maximum Capability. It is used in some internal functions.
    InternalSuperUser,
    /// Participant of Veracruz identified by ID
    Participant(u64),
    /// Program in Veracruz, identified by the program file name.
    Program(String),
    /// Pipeline in Veracruz, identified by the pipeline name.
    Pipeline(String),
    /// Native module in Veracruz, identified by its name.
    NativeModule(String),
    /// No Capability, the bottom Capability. It is used in some Initialization.
    NoCap,
}

/// The Right Table, contains the `Right`, i.e.
/// the allowed operations of a Principal on a file
pub type RightsTable = HashMap<Principal, HashMap<PathBuf, Rights>>;

/// Defines a file entry in the policy, containing the name and `Right`, the allowed op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileRights {
    /// The file name
    file_name: String,
    /// The associated right, when someone open the file
    rights: u32,
}

impl FileRights {
    /// Creates a new file permission.
    #[inline]
    pub fn new(file_name: String, rights: u32) -> Self {
        Self { file_name, rights }
    }

    /// Returns the file_name.
    #[inline]
    pub fn file_name(&self) -> &str {
        self.file_name.as_str()
    }

    /// Returns the rights.
    #[inline]
    pub fn rights(&self) -> &u32 {
        &self.rights
    }

    /// Convert a vec of FileRights to a Hashmap from filenames to Rights.
    #[inline]
    pub fn compute_right_map(file_right_vec: &[FileRights]) -> HashMap<PathBuf, Rights> {
        file_right_vec.iter().fold(
            HashMap::new(),
            |mut acc, FileRights { file_name, rights }| {
                acc.insert(file_name.into(), Rights::from_bits_truncate(*rights as u64));
                acc
            },
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// Program
////////////////////////////////////////////////////////////////////////////////
/// Defines a program that can be loaded into execution engine.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// The file name
    program_file_name: String,
    /// The program ID
    id: u32,
    /// The file permission that specifies the program's ability to read, write and execute files.
    file_rights: Vec<FileRights>,
}

impl Program {
    /// Creates a veracruz program.
    #[inline]
    pub fn new<T: Into<u32>>(
        program_file_name: String,
        id: T,
        file_rights: Vec<FileRights>,
    ) -> Self {
        Self {
            program_file_name,
            id: id.into(),
            file_rights,
        }
    }

    /// Return the program file name.
    #[inline]
    pub fn program_file_name(&self) -> &str {
        self.program_file_name.as_str()
    }

    /// Return the program id.
    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Return file rights map associated to the program.
    #[inline]
    pub fn file_rights_map(&self) -> HashMap<PathBuf, Rights> {
        FileRights::compute_right_map(&self.file_rights)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Native module
////////////////////////////////////////////////////////////////////////////////
/// Defines a native module that can be loaded. Encompasses both static and
/// dynamic native modules, which are respectively part of the Veracruz runtime
/// and separate binaries, both executed on invocation by the WASM program
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeModule {
    /// Native module's name
    name: String,
    /// Dynamic native module's entry point, i.e. path to the main binary
    /// relative to the native module's root directory. If set to `None`, the
    /// native module is assumed to be static and is searched for in the list of
    /// static native modules
    entry_point_path: PathBuf,
    /// Path to native module's special file. Writing data to this file triggers
    /// the execution of the native module with data as input
    special_file_path: PathBuf,
    /// Native module's ID
    id: u32,
    // TODO: add sandbox policy
}

impl NativeModule {
    /// Creates a Veracruz native module.
    #[inline]
    pub fn new<T: Into<u32>>(name: String, entry_point_path: PathBuf, special_file_path: PathBuf, id: T) -> Self
    {
        Self {
            name,
            entry_point_path,
            special_file_path,
            id: id.into(),
        }
    }

    /// Return the name.
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return path to entry point.
    #[inline]
    pub fn entry_point_path(&self) -> &PathBuf {
        &self.entry_point_path
    }

    /// Return path to special file.
    #[inline]
    pub fn special_file_path(&self) -> &PathBuf {
        &self.special_file_path
    }

    /// Return the native module's id.
    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Return whether the native module is static or dynamic.
    #[inline]
    pub fn is_static(&self) -> bool {
        self.entry_point_path() == &PathBuf::from("")
    }
}

impl Debug for NativeModule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\"{}\" special_file=\"{}\" entry_point=", self.name(), self.special_file_path().to_str().unwrap_or_default())?;
        if self.is_static() {
            write!(f, "static")
        } else {
            write!(f, "\"{}\"", self.entry_point_path().to_str().unwrap_or_default())
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Pipeline
////////////////////////////////////////////////////////////////////////////////
/// Defines a program that can be loaded into execution engine.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pipeline {
    /// The pipeline name
    name: String,
    /// The script
    #[serde(rename = "pipeline")]
    preparsed_pipeline: String,
    /// The parsed, AST representation of the conditional pipeline of programs
    /// to execute.  This is not present in the JSON representation of a policy
    /// file.
    #[serde(skip)]
    parsed_pipeline: Option<Box<Expr>>,
    /// The pipeline ID
    id: u32,
    /// The file permission that specifies the program's ability to read, write and execute files.
    file_rights: Vec<FileRights>,
}

impl Pipeline {
    /// Creates a veracruz pipeline.
    #[inline]
    pub fn new<T: Into<u32>>(
        name: String,
        id: T,
        preparsed_pipeline: String,
        file_rights: Vec<FileRights>,
    ) -> Result<Self> {
        let parsed_pipeline = Some(parse_pipeline(&preparsed_pipeline)?);
        Ok(Self {
            name,
            id: id.into(),
            parsed_pipeline,
            preparsed_pipeline,
            file_rights,
        })
    }

    /// Parse the pipeline.
    #[inline]
    pub fn parse(&mut self) -> Result<()> {
        if let None = self.parsed_pipeline {
            self.parsed_pipeline = Some(parse_pipeline(&self.preparsed_pipeline)?);
        }
        Ok(())
    }

    /// Return the name of the pipeline.
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return file rights map associated to the program.
    #[inline]
    pub fn file_rights_map(&self) -> HashMap<PathBuf, Rights> {
        FileRights::compute_right_map(&self.file_rights)
    }

    /// Return the pipeline AST.
    #[inline]
    pub fn get_parsed_pipeline(&self) -> Result<&Box<Expr>> {
        self.parsed_pipeline
            .as_ref()
            .ok_or(anyhow!("The pipeline is not parsed"))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Execution strategies.
////////////////////////////////////////////////////////////////////////////////

/// Defines the execution strategy that will be used to execute the WASM
/// program.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ExecutionStrategy {
    /// Interpretation will be used to execute the WASM binary.
    Interpretation,
    /// JIT compilation will be used to execute the WASM binary.
    JIT,
}

////////////////////////////////////////////////////////////////////////////////
// Identities.
////////////////////////////////////////////////////////////////////////////////

/// A notion of identity for Veracruz principals.  Note that in different
/// contexts we require different representations from our cryptographic
/// certificates: in some contexts these should be unparsed text representations
/// of the certificates (e.g. in the material below), and in other circumstances
/// a parsed format is more appropriate, e.g. the `Certificate` type from the
/// `Mbed TLS` library, as used by the session manager.  We therefore abstract
/// over the concrete types of certificates to obtain a single type that suits
/// both contexts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity<U> {
    /// The cryptographic certificate associated with this identity.  Note that
    /// the actual implementation of this is kept abstract.
    certificate: U,
    /// The ID associated with this identity.
    /// TODO: what is this?  Explain it properly.
    id: u32,
    /// The file capabilities that specifies this principal's ability to read,
    /// write and execute files.
    file_rights: Vec<FileRights>,
}

impl<U> Identity<U> {
    /// Creates a new identity from a certificate, and identifier.  Initially,
    /// we keep the set of roles empty.
    #[inline]
    pub fn new<T>(certificate: U, id: T, file_rights: Vec<FileRights>) -> Self
    where
        T: Into<u32>,
    {
        Self {
            certificate,
            id: id.into(),
            file_rights,
        }
    }

    /// Return file rights map associated to the program.
    #[inline]
    pub fn file_rights(&self) -> &Vec<FileRights> {
        &self.file_rights
    }

    /// Return file rights map associated to the program.
    #[inline]
    pub fn file_rights_map(&self) -> HashMap<PathBuf, Rights> {
        FileRights::compute_right_map(&self.file_rights)
    }

    /// Returns the certificate associated with this identity.
    #[inline]
    pub fn certificate(&self) -> &U {
        &self.certificate
    }

    /// Returns the ID associated with this identity.
    #[inline]
    pub fn id(&self) -> &u32 {
        &self.id
    }
}

impl Identity<String> {
    /// Checks the validity of the identity, including well-formedness checks on
    /// the structure of the X509 certificate.  Returns `Err(reason)` iff the
    /// identity is malformed.  Returns `Ok(())` in all other cases.
    ///
    /// NOTE: the X509 apparently does not check the end of certificates for a
    /// valid certificate termination line.  As a result, we check that in this
    /// function.
    pub fn assert_valid(&self) -> Result<()> {
        if !self.certificate().ends_with("-----END CERTIFICATE-----") {
            return Err(anyhow!(PolicyError::FormatError));
        }

        #[cfg(features = "std")]
        {
            use mbedtls::x509::Certificate;
            use veracruz_utils::csr::generate_x509_time_now;

            let mut buffer: Vec<u8> = self.certificate().as_bytes().to_vec();
            buffer.push(b'\0');
            let cert = Certificate::from_pem(&buffer)?;
            let not_before = cert.not_before()?.to_x509_time();
            let not_after = cert.not_after()?.to_x509_time();
            let now = generate_x509_time_now();
            if now < not_before || now > not_after {
                return Err(anyhow!(PolicyError::FormatError));
            }
        }

        Ok(())
    }
}

/// Defines a file and its expected hash.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileHash {
    file_path: String,
    hash: String,
}

impl FileHash {
    /// Creates a new file permission.
    #[inline]
    pub fn new(file_path: String, hash: String) -> Self {
        Self { file_path, hash }
    }

    /// Returns the file_name.
    #[inline]
    pub fn file_path(&self) -> &str {
        self.file_path.as_str()
    }

    /// Returns the rights.
    #[inline]
    pub fn hash(&self) -> &str {
        &self.hash.as_str()
    }
}
