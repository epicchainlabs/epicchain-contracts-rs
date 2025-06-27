# EpicChain Syscall Implementation Details

This document describes the implementation of EpicChain syscall support in the epicchain-contract-rs framework.

## Overview

The epicchain-contract-rs framework allows Rust smart contracts to interact with the EpicChain blockchain through syscalls. These syscalls are defined in the EpicChain protocol and provide access to blockchain features such as storage, runtime information, and other blockchain services.

## Architecture

The syscall implementation consists of the following components:

1. **Rust Interface**: Defined in `neo-contract/src/env/syscall.rs`
2. **Runtime Wrappers**: Helper functions in `neo-contract/src/runtime/mod.rs` that provide a more idiomatic Rust API
3. **Syscall Mappings**: Defined in `epicchain-wasm/rosetta/builtin/syscall.go`, mapping Rust function names to EpicChain syscall names
4. **Translation Logic**: Implemented in `epicchain-wasm/rosetta/translates/syscall.go`, which converts WASM imports to NEF syscall instructions

## Syscall Flow

1. A Rust contract calls a wrapper function from the `runtime` module
2. The wrapper calls an unsafe function from `env::syscall`
3. During compilation to WASM, these calls become WASM imports with the "neo.syscall" module
4. The epicchain-wasm compiler recognizes these imports and maps them to EpicChain VM syscalls
5. In the generated NEF bytecode, these are converted to SYSCALL opcodes with the appropriate system call name

## Implementation Details

### 1. Rust Syscall Interface

The syscall interface is defined in `env/syscall.rs` with external functions:

```rust
#[link(wasm_import_module = "neo.syscall")]
extern "C" {
    pub(crate) fn system_runtime_trigger() -> TriggerType;
    pub(crate) fn system_runtime_platform() -> ByteString;
    // ... other syscalls
}
```

### 2. Rust Runtime Wrappers

Idiomatic wrapper functions are provided in `runtime/mod.rs`:

```rust
pub fn get_time() -> u64 {
    unsafe { env::syscall::system_runtime_time() }
}

pub fn get_platform() -> ByteString {
    unsafe { env::syscall::system_runtime_platform() }
}
```

### 3. Syscall Name Mapping

In `rosetta/builtin/syscall.go`, each Rust syscall is mapped to its EpicChain VM counterpart:

```go
syscalls = map[string]Syscall{
    "system_runtime_trigger": {"System.Runtime.Trigger", []neo.ParamType{}, neo.ParamInteger},
    "system_runtime_platform": {"System.Runtime.Platform", []neo.ParamType{}, neo.ParamString},
    // ... other mappings
}
```

### 4. WASM Import Processing

The compiler processes WASM imports in `rosetta.go`:

```go
// Process built-in functions from imports
for module, imports := range module.Import.GroupByModule() {
    for _, entry := range imports {
        if entry.Kind != wasm.ImportFunctionKind {
            continue
        }

        importName := entry.Field

        // Handle syscall module imports
        if builtin.IsSyscallModule(module) {
            // Create a translation for the syscall function
            syscallTranslation := translates.NewSyscall()
            translation, err := syscallTranslation.Translate(nil, importName)
            // ... store the translation
        }
    }
}
```

### 5. Syscall Translation

In `translates/syscall.go`, the actual SYSCALL opcode generation happens:

```go
func (s *SyscallTranslation) Translate(reader *op.OpsReader, importName string) (*Translation, error) {
    // Get the corresponding syscall
    syscall, ok := builtin.GetSyscallFromImport(importName)
    if !ok {
        return nil, fmt.Errorf("unknown syscall: %s", importName)
    }

    // Calculate the syscall hash (first 4 bytes of SHA256 hash of the syscall name)
    syscallNameBytes := []byte(syscall.Name)
    syscallHash := sha256Hash(syscallNameBytes)[:4]

    s.logVerbose("Syscall hash for '%s': %x", syscall.Name, syscallHash)

    // Create a translation with SYSCALL operation
    translation := &Translation{
        Sources: []WasmOp{},
        Targets: []neo.VmOp{
            {
                OpCode: neo.OpSyscall,
                First:  syscallHash, // Use the first 4 bytes of the SHA256 hash
            },
        },
    }

    return translation, nil
}
```

## Supported Syscalls

The current implementation supports all standard EpicChain syscalls, including:

### Runtime
- System.Runtime.Trigger
- System.Runtime.Platform
- System.Runtime.GetScriptContainer
- System.Runtime.GetExecutingScriptHash
- System.Runtime.GetCallingScriptHash
- System.Runtime.GetEntryScriptHash
- System.Runtime.GetTime
- System.Runtime.GetInvocationCounter
- System.Runtime.EpicPulseLeft
- System.Runtime.GetAddressVersion
- System.Runtime.GetNotifications
- System.Runtime.CheckWitness
- System.Runtime.Log
- System.Runtime.BurnEpicPulse
- System.Runtime.GetRandom
- System.Runtime.GetNetwork
- System.Runtime.LoadScript
- System.Runtime.GetCurrentSigners

### Contract
- System.Contract.Call
- System.Contract.GetCallFlags
- System.Contract.CreateStandardAccount
- System.Contract.CreateMultisigAccount

### Crypto
- System.Crypto.CheckSig
- System.Crypto.CheckMultisig

### Iterator
- System.Iterator.Next
- System.Iterator.Value

### Storage
- System.Storage.GetContext
- System.Storage.GetReadOnlyContext
- System.Storage.AsReadOnly
- System.Storage.Get
- System.Storage.Put
- System.Storage.Delete
- System.Storage.Find

## Debugging Syscalls

When debugging syscall issues:

1. Enable verbose logging: `EPICCHAIN_WASM_DEBUG=1 EPICCHAIN_WASM_VERBOSE=1`
2. Check for "Processing syscall import" messages in the logs
3. Verify the syscall name in the generated NEF assembly
4. Confirm that the correct System.X.Y methods are being called

## Future Improvements

1. Add more detailed logging for syscall conversions
2. Implement caching of frequently used syscalls
3. Add validation of syscall parameters to catch errors early
4. Support for encoding custom syscall attributes in the manifest