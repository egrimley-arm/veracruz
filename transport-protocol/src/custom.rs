//! Custom and derived functionality relating to the transport protocol.
//!
//! ## Authors
//!
//! The Veracruz Development Team.
//!
//! ## Licensing and copyright notice
//!
//! See the `LICENSE_MIT.markdown` file in the Veracruz root directory for
//! information on licensing and copyright.

use crate::transport_protocol;
use err_derive::Error;
use protobuf::{error::ProtobufError, Message, ProtobufEnum};
use std::{result::Result, string::ToString};

pub const LENGTH_PREFIX_SIZE: usize = 8;

#[derive(Debug, Error)]
pub enum TransportProtocolError {
    // NOTE: Protobuf does not implement clone, hence derive(clone) is impossible.
    #[error(display = "TransportProtocol: ProtobufError: {:?}.", _0)]
    ProtobufError(#[error(source)] ProtobufError),
    #[error(display = "TransportProtocol: Invalid response status: {:?}.", _0)]
    ResponseStatusError(i32),
    #[error(display = "TransportProtocol: TryIntoError: {}.", _0)]
    TryIntoError(#[error(source)] std::num::TryFromIntError),
}
type TransportProtocolResult = Result<std::vec::Vec<u8>, TransportProtocolError>;

// Strip length prefix from protocol buffer.
// Return length and stripped protocol buffer.
// This function must be called before deserializing a message
pub fn get_length_prefix(buffer: &[u8]) -> (u64, &[u8]) {
    let mut length_bytes: [u8; LENGTH_PREFIX_SIZE] = [0; LENGTH_PREFIX_SIZE];
    length_bytes.copy_from_slice(&buffer[..LENGTH_PREFIX_SIZE]);
    let remaining_buffer = &buffer[LENGTH_PREFIX_SIZE..];
    (u64::from_be_bytes(length_bytes), remaining_buffer)
}

// Return protocol buffer prefixed with its length.
// This function must be called after serializing a message
pub fn set_length_prefix(buffer: &mut Vec<u8>) -> TransportProtocolResult {
    let length = u64::to_be_bytes(buffer.len() as u64);
    let mut length_bytes = length.to_vec();
    length_bytes.append(buffer);
    Ok(length_bytes)
}

/// Parse a request to the Runtime Manager.
pub fn parse_runtime_manager_request(
    buffer: &[u8],
) -> Result<transport_protocol::RuntimeManagerRequest, TransportProtocolError> {
    // Strip length prefix
    let (_length, buffer_unprefixed) = get_length_prefix(&buffer);

    Ok(protobuf::parse_from_bytes::<
        transport_protocol::RuntimeManagerRequest,
    >(buffer_unprefixed)?)
}

/// Parse a response from the Runtime Manager.
pub fn parse_runtime_manager_response(
    buffer: &[u8],
) -> Result<transport_protocol::RuntimeManagerResponse, TransportProtocolError> {
    // Strip length prefix
    let (_length, buffer_unprefixed) = get_length_prefix(&buffer);

    Ok(protobuf::parse_from_bytes::<
        transport_protocol::RuntimeManagerResponse,
    >(buffer_unprefixed)?)
}

pub fn parse_proxy_attestation_server_request(
    buffer: &[u8],
) -> Result<transport_protocol::ProxyAttestationServerRequest, TransportProtocolError> {
    // Strip length prefix
    let (_length, buffer_unprefixed) = get_length_prefix(&buffer);

    Ok(protobuf::parse_from_bytes::<
        transport_protocol::ProxyAttestationServerRequest,
    >(buffer_unprefixed)?)
}

pub fn parse_proxy_attestation_server_response(
    buffer: &[u8],
) -> Result<transport_protocol::ProxyAttestationServerResponse, TransportProtocolError> {
    // Strip length prefix
    let (_length, buffer_unprefixed) = get_length_prefix(&buffer);

    Ok(protobuf::parse_from_bytes::<
        transport_protocol::ProxyAttestationServerResponse,
    >(buffer_unprefixed)?)
}

/// Serialize a program binary.
pub fn serialize_program(program_buffer: &[u8], file_name: &str) -> TransportProtocolResult {
    let mut program = transport_protocol::Data::new();
    program.set_data(program_buffer.to_vec());
    program.set_file_name(file_name.to_string());
    let mut abs = transport_protocol::RuntimeManagerRequest::new();
    abs.set_write_file(program);

    // Prefix buffer with its length
    let mut buffer = abs.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize a (static) data package and its package ID.
pub fn serialize_program_data(data_buffer: &[u8], file_name: &str) -> TransportProtocolResult {
    let mut data = transport_protocol::Data::new();
    data.set_data(data_buffer.to_vec());
    data.set_file_name(file_name.to_string());
    let mut transport_protocol = transport_protocol::RuntimeManagerRequest::new();
    transport_protocol.set_write_file(data);

    // Prefix buffer with its length
    let mut buffer = transport_protocol.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize a (static) data package and its package ID.
pub fn serialize_write_file(data_buffer: &[u8], file_name: &str) -> TransportProtocolResult {
    let mut data = transport_protocol::Data::new();
    data.set_data(data_buffer.to_vec());
    data.set_file_name(file_name.to_string());
    let mut transport_protocol = transport_protocol::RuntimeManagerRequest::new();
    transport_protocol.set_write_file(data);

    // Prefix buffer with its length
    let mut buffer = transport_protocol.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize a (static) data package and its package ID.
pub fn serialize_read_file(file_name: &str) -> TransportProtocolResult {
    let mut data = transport_protocol::Read::new();
    data.set_file_name(file_name.to_string());
    let mut transport_protocol = transport_protocol::RuntimeManagerRequest::new();
    transport_protocol.set_read_file(data);

    // Prefix buffer with its length
    let mut buffer = transport_protocol.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize a stream data package and its package ID.
pub fn serialize_stream(data_buffer: &[u8], file_name: &str) -> TransportProtocolResult {
    let mut data = transport_protocol::Data::new();
    data.set_data(data_buffer.to_vec());
    data.set_file_name(file_name.to_string());
    let mut transport_protocol = transport_protocol::RuntimeManagerRequest::new();
    transport_protocol.set_append_file(data);

    // Prefix buffer with its length
    let mut buffer = transport_protocol.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize the request for querying the result.
pub fn serialize_request_result(file_name: &str) -> TransportProtocolResult {
    let mut command = transport_protocol::RequestResult::new();
    command.set_file_name(file_name.to_string());
    let mut request = transport_protocol::RuntimeManagerRequest::new();
    request.set_request_result(command);

    // Prefix buffer with its length
    let mut buffer = request.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize the request for shutting down the enclave.
pub fn serialize_request_shutdown() -> TransportProtocolResult {
    let command = transport_protocol::RequestShutdown::new();
    let mut request = transport_protocol::RuntimeManagerRequest::new();
    request.set_request_shutdown(command);

    // Prefix buffer with its length
    let mut buffer = request.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

pub fn serialize_request_proxy_psa_attestation_token(challenge: &[u8]) -> TransportProtocolResult {
    let mut rpat = transport_protocol::RequestProxyPsaAttestationToken::new();
    rpat.set_challenge(challenge.to_vec());
    let mut request = transport_protocol::RuntimeManagerRequest::new();
    request.set_request_proxy_psa_attestation_token(rpat);

    // Prefix buffer with its length
    let mut buffer = request.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

pub fn parse_request_proxy_psa_attestation_token(
    proto: &transport_protocol::RequestProxyPsaAttestationToken,
) -> std::vec::Vec<u8> {
    proto.get_challenge().to_vec()
}

pub fn parse_cert_chain(
    chain: &transport_protocol::CertChain,
) -> (std::vec::Vec<u8>, std::vec::Vec<u8>) {
    return (
        chain.get_root_cert().to_vec(),
        chain.get_enclave_cert().to_vec(),
    );
}

pub fn serialize_proxy_psa_attestation_token(
    token: &[u8],
    pubkey: &[u8],
    device_id: i32,
) -> TransportProtocolResult {
    let mut pat_proto = transport_protocol::ProxyPsaAttestationToken::new();
    pat_proto.set_token(token.to_vec());
    pat_proto.set_pubkey(pubkey.to_vec());
    pat_proto.set_device_id(device_id);
    let mut proxy_attestation_server_request =
        transport_protocol::ProxyAttestationServerRequest::new();
    proxy_attestation_server_request.set_proxy_psa_attestation_token(pat_proto);

    // Prefix buffer with its length
    let mut buffer = proxy_attestation_server_request.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

pub fn serialize_nitro_attestation_doc(doc: &[u8], device_id: i32) -> TransportProtocolResult {
    let mut nad_proto = transport_protocol::NitroAttestationDoc::new();
    nad_proto.set_doc(doc.to_vec());
    nad_proto.set_device_id(device_id);
    let mut proxy_attestation_server_request =
        transport_protocol::ProxyAttestationServerRequest::new();
    proxy_attestation_server_request.set_nitro_attestation_doc(nad_proto);

    // Prefix buffer with its length
    let mut buffer = proxy_attestation_server_request.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

pub fn serialize_certificate(cert: &[u8]) -> TransportProtocolResult {
    let mut proto_cert = transport_protocol::Cert::new();
    proto_cert.set_data(cert.to_vec());
    let mut rmr = transport_protocol::RuntimeManagerResponse::new();
    rmr.set_cert(proto_cert);

    // Prefix buffer with its length
    let mut buffer = rmr.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

pub fn parse_proxy_psa_attestation_token(
    proto: &transport_protocol::ProxyPsaAttestationToken,
) -> (std::vec::Vec<u8>, std::vec::Vec<u8>, i32) {
    (
        proto.get_token().to_vec(),
        proto.get_pubkey().to_vec(),
        proto.get_device_id(),
    )
}

pub fn serialize_native_psa_attestation_token(
    token: &[u8],
    csr: &[u8],
    device_id: i32,
) -> TransportProtocolResult {
    let mut pat_proto = transport_protocol::NativePsaAttestationToken::new();
    pat_proto.set_token(token.to_vec());
    pat_proto.set_csr(csr.to_vec());
    pat_proto.set_device_id(device_id);
    let mut proxy_attestation_server_request =
        transport_protocol::ProxyAttestationServerRequest::new();
    proxy_attestation_server_request.set_native_psa_attestation_token(pat_proto);

    // Prefix buffer with its length
    let mut buffer = proxy_attestation_server_request.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

pub fn parse_native_psa_attestation_token(
    proto: &transport_protocol::NativePsaAttestationToken,
) -> (std::vec::Vec<u8>, std::vec::Vec<u8>, i32) {
    (
        proto.get_token().to_vec(),
        proto.get_csr().to_vec(),
        proto.get_device_id(),
    )
}

pub fn parse_nitro_attestation_doc(
    proto: &transport_protocol::NitroAttestationDoc,
) -> (std::vec::Vec<u8>, i32) {
    (proto.get_doc().to_vec(), proto.get_device_id())
}

pub fn serialize_cert_chain(enclave_cert: &[u8], root_cert: &[u8]) -> TransportProtocolResult {
    let mut cert_chain = transport_protocol::CertChain::new();
    cert_chain.set_root_cert(root_cert.to_vec());
    cert_chain.set_enclave_cert(enclave_cert.to_vec());
    let mut response = transport_protocol::ProxyAttestationServerResponse::new();
    response.set_cert_chain(cert_chain);

    // Prefix buffer with its length
    let mut buffer = response.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

pub fn serialize_psa_attestation_init(challenge: &[u8], device_id: i32) -> TransportProtocolResult {
    let mut request = transport_protocol::ProxyAttestationServerResponse::new();
    let mut pai = transport_protocol::PsaAttestationInit::new();
    pai.set_challenge(challenge.to_vec());
    pai.set_device_id(device_id);
    request.set_psa_attestation_init(pai);

    // Prefix buffer with its length
    let mut buffer = request.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

pub fn parse_psa_attestation_init(
    pai: &transport_protocol::PsaAttestationInit,
) -> Result<(std::vec::Vec<u8>, i32), TransportProtocolError> {
    Ok((pai.get_challenge().to_vec(), pai.get_device_id()))
}

/// Serialize the request for querying the hash of the provisioned program.
#[deprecated]
pub fn serialize_request_pi_hash(file_name: &str) -> TransportProtocolResult {
    let mut request = transport_protocol::RuntimeManagerRequest::new();
    let mut rph = transport_protocol::RequestPiHash::new();
    rph.set_file_name(file_name.to_string());
    request.set_request_pi_hash(rph);

    // Prefix buffer with its length
    let mut buffer = request.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize the request for querying the enclave policy.
pub fn serialize_request_policy_hash() -> TransportProtocolResult {
    let mut request = transport_protocol::RuntimeManagerRequest::new();
    let rph = transport_protocol::RequestPolicyHash::new();
    request.set_request_policy_hash(rph);

    // Prefix buffer with its length
    let mut buffer = request.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize the request for querying state of the enclave.
pub fn serialize_machine_state(machine_state: u8) -> TransportProtocolResult {
    let mut response = transport_protocol::RuntimeManagerResponse::new();

    response.set_status(transport_protocol::ResponseStatus::SUCCESS);
    let mut state = transport_protocol::State::new();
    let slice = &vec![machine_state];

    state.state.resize(slice.len(), 0);
    state.state.copy_from_slice(slice);
    response.set_state(state);

    // Prefix buffer with its length
    let mut buffer = response.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize a response containing the program hash.
pub fn serialize_pi_hash(hash: &[u8]) -> TransportProtocolResult {
    let mut response = transport_protocol::RuntimeManagerResponse::new();

    response.set_status(transport_protocol::ResponseStatus::SUCCESS);
    let mut pi_hash = transport_protocol::PiHash::new();
    pi_hash.data.resize(hash.len(), 0);
    pi_hash.data.copy_from_slice(hash);
    response.set_pi_hash(pi_hash);

    // Prefix buffer with its length
    let mut buffer = response.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize a response containing the policy hash.
pub fn serialize_policy_hash(hash: &[u8]) -> TransportProtocolResult {
    let mut response = transport_protocol::RuntimeManagerResponse::new();

    response.set_status(transport_protocol::ResponseStatus::SUCCESS);
    let mut policy_hash = transport_protocol::PolicyHash::new();
    policy_hash.data.resize(hash.len(), 0);
    policy_hash.data.copy_from_slice(hash);
    response.set_policy_hash(policy_hash);

    // Prefix buffer with its length
    let mut buffer = response.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize an empty response.
pub fn serialize_empty_response(status: i32) -> TransportProtocolResult {
    let mut response = transport_protocol::RuntimeManagerResponse::new();
    let encoded_status = transport_protocol::ResponseStatus::from_i32(status)
        .ok_or(TransportProtocolError::ResponseStatusError(status))?;
    response.set_status(encoded_status);

    // Prefix buffer with its length
    let mut buffer = response.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

/// Serialize a response containing the computation result.
pub fn serialize_result(
    status: i32,
    data_opt: Option<std::vec::Vec<u8>>,
) -> TransportProtocolResult {
    let mut response = transport_protocol::RuntimeManagerResponse::new();

    let encoded_status = transport_protocol::ResponseStatus::from_i32(status)
        .ok_or(TransportProtocolError::ResponseStatusError(status))?;

    response.set_status(encoded_status);

    if let Some(ref data) = data_opt {
        let mut result = transport_protocol::Result::new();
        result.data.resize(data.len(), 0);
        result.data.copy_from_slice(&data);
        response.set_result(result);
    }

    // Prefix buffer with its length
    let mut buffer = response.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}

pub fn parse_result(
    response: &transport_protocol::RuntimeManagerResponse,
) -> Result<Option<std::vec::Vec<u8>>, TransportProtocolError> {
    let status = response.get_status();
    let decoded_status = match status {
        transport_protocol::ResponseStatus::UNSET => -1,
        transport_protocol::ResponseStatus::SUCCESS => 0,
        transport_protocol::ResponseStatus::FAILED_INVALID_ROLE => 1,
        transport_protocol::ResponseStatus::FAILED_NOT_READY => 2,
        transport_protocol::ResponseStatus::FAILED_GENERIC => 3,
        transport_protocol::ResponseStatus::FAILED_VM_ERROR => 4,
        transport_protocol::ResponseStatus::FAILED_ERROR_CODE_RETURNED => 5,
        transport_protocol::ResponseStatus::FAILED_INVALID_REQUEST => 6,
    };
    if status != transport_protocol::ResponseStatus::SUCCESS {
        return Err(TransportProtocolError::ResponseStatusError(decoded_status));
    }

    let data_opt = {
        if response.has_result() {
            let result = response.get_result();
            let mut data = std::vec::Vec::new();
            data.resize(result.get_data().len(), 0);
            data.copy_from_slice(&response.get_result().data);
            Some(data)
        } else {
            None
        }
    };

    Ok(data_opt)
}

pub fn parse_start_msg(
    parsed: &transport_protocol::ProxyAttestationServerRequest,
) -> (std::string::String, std::string::String) {
    let start_msg = parsed.get_start_msg();
    (
        start_msg.protocol.clone(),
        start_msg.firmware_version.clone(),
    )
}

pub fn serialize_start_msg(protocol: &str, firmware_version: &str) -> TransportProtocolResult {
    let mut transport_protocol = transport_protocol::ProxyAttestationServerRequest::new();
    let mut start_msg = transport_protocol::StartMsg::new();
    start_msg.set_protocol(protocol.to_string());
    start_msg.set_firmware_version(firmware_version.to_string());
    transport_protocol.set_start_msg(start_msg);

    // Prefix buffer with its length
    let mut buffer = transport_protocol.write_to_bytes()?;
    set_length_prefix(&mut buffer)
}
