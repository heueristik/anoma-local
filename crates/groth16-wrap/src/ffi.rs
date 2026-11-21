//! The C interface of the Go library in `go/ffi`. All `unsafe` code of the crate is in this module.
//!
//! Each Go function copies its inputs before it returns and keeps no pointer to them. It writes each output into a
//! buffer that it allocates with `malloc`, and the caller frees the buffer with `groth16_wrap_free`.

use std::ptr;
use std::slice;

use crate::Error;

const OK: i32 = 0;
const REJECTED: i32 = 1;
const FAILED: i32 = 2;

/// `groth16_wrap_buffer` of the Go library.
#[repr(C)]
struct RawBuffer {
    data: *mut u8,
    length: usize,
}

unsafe extern "C" {
    fn groth16_wrap_single_party_setup(
        inner_verifying_key: *const u8,
        inner_verifying_key_length: usize,
        directory: *const u8,
        directory_length: usize,
        message: *mut RawBuffer,
    ) -> i32;

    fn groth16_wrap_prove(
        directory: *const u8,
        directory_length: usize,
        inner_proof: *const u8,
        inner_proof_length: usize,
        public_inputs: *const u8,
        public_inputs_length: usize,
        proof: *mut RawBuffer,
        proof_public_inputs: *mut RawBuffer,
        message: *mut RawBuffer,
    ) -> i32;

    fn groth16_wrap_verify(
        directory: *const u8,
        directory_length: usize,
        proof: *const u8,
        proof_length: usize,
        public_inputs: *const u8,
        public_inputs_length: usize,
        message: *mut RawBuffer,
    ) -> i32;

    fn groth16_wrap_solidity_verifier(
        directory: *const u8,
        directory_length: usize,
        source: *mut RawBuffer,
        message: *mut RawBuffer,
    ) -> i32;

    fn groth16_wrap_free(data: *mut u8);
}

pub(crate) fn single_party_setup(
    inner_verifying_key: &[u8],
    directory: &[u8],
) -> Result<(), Error> {
    let mut message = GoBuffer::new();
    // SAFETY: each pointer and length pair comes from a live slice, and `message` is a live, empty buffer.
    let status = unsafe {
        groth16_wrap_single_party_setup(
            inner_verifying_key.as_ptr(),
            inner_verifying_key.len(),
            directory.as_ptr(),
            directory.len(),
            message.as_mut_ptr(),
        )
    };
    check(status, &message)
}

pub(crate) fn prove(
    directory: &[u8],
    inner_proof: &[u8],
    public_inputs: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let mut proof = GoBuffer::new();
    let mut proof_public_inputs = GoBuffer::new();
    let mut message = GoBuffer::new();
    // SAFETY: each pointer and length pair comes from a live slice, and each out pointer is a live, empty buffer.
    let status = unsafe {
        groth16_wrap_prove(
            directory.as_ptr(),
            directory.len(),
            inner_proof.as_ptr(),
            inner_proof.len(),
            public_inputs.as_ptr(),
            public_inputs.len(),
            proof.as_mut_ptr(),
            proof_public_inputs.as_mut_ptr(),
            message.as_mut_ptr(),
        )
    };
    check(status, &message)?;
    Ok((proof.to_vec(), proof_public_inputs.to_vec()))
}

pub(crate) fn verify(directory: &[u8], proof: &[u8], public_inputs: &[u8]) -> Result<(), Error> {
    let mut message = GoBuffer::new();
    // SAFETY: each pointer and length pair comes from a live slice, and `message` is a live, empty buffer.
    let status = unsafe {
        groth16_wrap_verify(
            directory.as_ptr(),
            directory.len(),
            proof.as_ptr(),
            proof.len(),
            public_inputs.as_ptr(),
            public_inputs.len(),
            message.as_mut_ptr(),
        )
    };
    check(status, &message)
}

pub(crate) fn solidity_verifier(directory: &[u8]) -> Result<Vec<u8>, Error> {
    let mut source = GoBuffer::new();
    let mut message = GoBuffer::new();
    // SAFETY: the pointer and length pair comes from a live slice, and each out pointer is a live, empty buffer.
    let status = unsafe {
        groth16_wrap_solidity_verifier(
            directory.as_ptr(),
            directory.len(),
            source.as_mut_ptr(),
            message.as_mut_ptr(),
        )
    };
    check(status, &message)?;
    Ok(source.to_vec())
}

fn check(status: i32, message: &GoBuffer) -> Result<(), Error> {
    match status {
        OK => Ok(()),
        REJECTED => Err(Error::Rejected(message.to_string_lossy())),
        FAILED => Err(Error::Failed(message.to_string_lossy())),
        unknown => Err(Error::UnknownStatus(unknown)),
    }
}

/// A buffer that the Go library fills. Drop frees its memory.
struct GoBuffer(RawBuffer);

impl GoBuffer {
    fn new() -> Self {
        Self(RawBuffer {
            data: ptr::null_mut(),
            length: 0,
        })
    }

    fn as_mut_ptr(&mut self) -> *mut RawBuffer {
        &mut self.0
    }

    fn as_slice(&self) -> &[u8] {
        if self.0.data.is_null() {
            return &[];
        }
        // SAFETY: Go wrote `length` initialized bytes at `data`, and they live until drop frees them.
        unsafe { slice::from_raw_parts(self.0.data, self.0.length) }
    }

    fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }

    fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.as_slice()).into_owned()
    }
}

impl Drop for GoBuffer {
    fn drop(&mut self) {
        // SAFETY: `data` is null or comes from `malloc` in Go and is not freed yet; `free(NULL)` does nothing.
        unsafe { groth16_wrap_free(self.0.data) }
    }
}
