// Command ffi exports package wrap to C. Build it with go build -buildmode=c-archive.
//
// Each function returns a status. On a status other than GROTH16_WRAP_OK, it writes the error text to message.
// The caller owns every buffer that a function writes and frees it with groth16_wrap_free.
package main

/*
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
	uint8_t *data;
	size_t length;
} groth16_wrap_buffer;

enum {
	GROTH16_WRAP_OK = 0,
	GROTH16_WRAP_REJECTED = 1,
	GROTH16_WRAP_FAILED = 2,
};
*/
import "C"

import (
	"bytes"
	"errors"
	"fmt"
	"runtime/debug"
	"unsafe"

	"github.com/consensys/gnark/logger"

	"github.com/wstran/ultrahonk-gnark/wrap"
)

func main() {}

// The status and the message report every error, so gnark does not write logs to the host process.
func init() {
	logger.Disable()
}

//export groth16_wrap_single_party_setup
func groth16_wrap_single_party_setup(innerVerifyingKey *C.uint8_t, innerVerifyingKeyLength C.size_t, directory *C.uint8_t, directoryLength C.size_t, message *C.groth16_wrap_buffer) C.int32_t {
	return run(message, func() error {
		return wrap.SinglePartySetup(goBytes(innerVerifyingKey, innerVerifyingKeyLength), goString(directory, directoryLength))
	})
}

//export groth16_wrap_prove
func groth16_wrap_prove(directory *C.uint8_t, directoryLength C.size_t, innerProof *C.uint8_t, innerProofLength C.size_t, publicInputs *C.uint8_t, publicInputsLength C.size_t, proof *C.groth16_wrap_buffer, proofPublicInputs *C.groth16_wrap_buffer, message *C.groth16_wrap_buffer) C.int32_t {
	return run(message, func() error {
		encodedProof, encodedInputs, err := wrap.Prove(goString(directory, directoryLength), goBytes(innerProof, innerProofLength), goBytes(publicInputs, publicInputsLength))
		if err != nil {
			return err
		}

		store(proof, encodedProof)
		store(proofPublicInputs, encodedInputs)

		return nil
	})
}

//export groth16_wrap_verify
func groth16_wrap_verify(directory *C.uint8_t, directoryLength C.size_t, proof *C.uint8_t, proofLength C.size_t, publicInputs *C.uint8_t, publicInputsLength C.size_t, message *C.groth16_wrap_buffer) C.int32_t {
	return run(message, func() error {
		return wrap.Verify(goString(directory, directoryLength), goBytes(proof, proofLength), goBytes(publicInputs, publicInputsLength))
	})
}

//export groth16_wrap_solidity_verifier
func groth16_wrap_solidity_verifier(directory *C.uint8_t, directoryLength C.size_t, source *C.groth16_wrap_buffer, message *C.groth16_wrap_buffer) C.int32_t {
	return run(message, func() error {
		encodedSource, err := wrap.SolidityVerifier(goString(directory, directoryLength))
		if err != nil {
			return err
		}

		store(source, encodedSource)

		return nil
	})
}

//export groth16_wrap_free
func groth16_wrap_free(data *C.uint8_t) {
	C.free(unsafe.Pointer(data))
}

// run calls operation, turns a panic into a failure, and returns the memory of the operation to the system.
func run(message *C.groth16_wrap_buffer, operation func() error) (status C.int32_t) {
	defer debug.FreeOSMemory()

	defer func() {
		if recovered := recover(); recovered != nil {
			status = report(message, fmt.Errorf("the wrap panicked: %v", recovered))
		}
	}()

	return report(message, operation())
}

func report(message *C.groth16_wrap_buffer, err error) C.int32_t {
	if err == nil {
		return C.GROTH16_WRAP_OK
	}

	store(message, []byte(err.Error()))

	var rejection *wrap.Rejection
	if errors.As(err, &rejection) {
		return C.GROTH16_WRAP_REJECTED
	}

	return C.GROTH16_WRAP_FAILED
}

// goBytes copies length bytes at data into Go memory.
func goBytes(data *C.uint8_t, length C.size_t) []byte {
	return bytes.Clone(unsafe.Slice((*byte)(unsafe.Pointer(data)), int(length)))
}

func goString(data *C.uint8_t, length C.size_t) string {
	return string(unsafe.Slice((*byte)(unsafe.Pointer(data)), int(length)))
}

// store copies data into memory from malloc, which the caller frees with groth16_wrap_free.
func store(buffer *C.groth16_wrap_buffer, data []byte) {
	buffer.data = (*C.uint8_t)(C.CBytes(data))
	buffer.length = C.size_t(len(data))
}
