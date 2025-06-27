# EpicChain Contract Manifest Generation

This document describes how manifest generation works in the **EpicChain Rust Smart Contract Framework** using the epicchain-wasm compiler.

## Overview

EpicChain smart contracts require a **manifest file** that defines the contract's metadata, permissions, and Application Binary Interface (ABI). This manifest is essential for the EpicChain VM to properly interact with the contract and for external applications to understand the contract's interface.

The framework **automatically generates** manifests using the **epicchain-wasm compiler** from:
1. **Compiled WASM file** - Contract bytecode analysis
2. **Rust source code** - Method signatures and documentation
3. **Contract attributes** - Metadata and permissions

## Build System Integration

### Automatic Generation with Makefiles

Every example includes manifest generation in the build process:

```bash
# Generate manifest using epicchain-wasm compiler
make manifest

# Or build everything (WASM → NEF + Manifest)
make
```

### EpicChain-WASM Compiler Commands

The manifest generation uses these epicchain-wasm commands:

```bash
# Generate manifest with source code integration
epicchain-wasm translate --input contract.wasm --manifest contract.manifest.json --source-code src/lib.rs

# Generate manifest from WASM only
epicchain-wasm translate --input contract.wasm --manifest contract.manifest.json
```

## Generation Process

### 1. WASM Analysis

The epicchain-wasm compiler analyzes the compiled WASM file to extract:

- **Exported functions** - Public contract methods
- **Function signatures** - Parameter types and return types
- **Method offsets** - Bytecode positions for each method

### 2. Source Code Integration

When source code is provided, the compiler enhances the manifest with:

- **Method documentation** - From Rust doc comments
- **Parameter names** - From function signatures
- **Contract metadata** - From contract attributes
- **Safety annotations** - Read-only method detection

### 3. Manifest Structure Creation

The compiler generates a complete EpicChain manifest with:

- **ABI definition** - Methods, parameters, return types
- **Contract metadata** - Name, author, version, description
- **Permissions** - Contract call permissions
- **Standards detection** - NEP-17, NEP-11 compliance

## Documentation Extraction

The manifest generator extracts documentation in the following ways:

1. **Contract Description**: Extracted from the doc comment on the contract's impl block
2. **Method Descriptions**: Extracted from doc comments on each exported method
3. **Parameter Names**: Obtained from the parameter names in the Rust function declarations
4. **Safety Information**: Extracted from `@safe` annotations in method doc comments

## Method Safety Determination

Methods are classified as "safe" (read-only) based on the following criteria, in order of precedence:

1. **`@safe` Annotation**: Methods with the `@safe` annotation in their doc comments are marked as safe
2. **Naming Conventions**: If no `@safe` annotation exists, methods are marked as safe based on these naming conventions:
   - Methods starting with `get_`, `query_`, `balance_`, or `total_`
   - Methods named `symbol`, `decimals`, or `contract_info`
3. **Default**: All other methods are considered potentially state-changing and marked as "unsafe"

Example of a safe method with annotation:

```rust
/// Returns the token symbol
///
/// @safe
pub fn symbol() -> String {
    "TOKEN".to_string()
}
```

## Standard Detection

The manifest generator automatically detects if a contract implements standard interfaces:

### NEP-17 (Fungible Token)
Detected when at least 3 of these methods are found:
- `transfer`
- `balanceOf`
- `totalSupply`
- `decimals`
- `symbol`

### NEP-11 (Non-Fungible Token)
Detected when any of these methods are found:
- `ownerOf`
- `tokens`
- `tokenURI`
- `transfer` (with specific signature)

## Manual Override

While the automatic generation should work well for most cases, developers can:

1. Add comprehensive documentation in their Rust code to enhance the generated manifest
2. Manually edit the generated manifest file if specific customization is needed
3. Use `@safe` annotations to explicitly mark methods as read-only

## Best Practices

For optimal manifest generation:

1. Add detailed doc comments to your contract impl block and exported methods
2. Follow naming conventions for read-only methods
3. Use `@safe` annotations for all read-only methods
4. Implement standard interfaces completely to ensure proper detection
5. Review the generated manifest file before deployment
6. Follow the [Code Documentation Style Guide](code-documentation-style.md) for consistent documentation

## Example Generated Manifest

Here's an example of a generated manifest for a NEP-17 token:

```json
{
  "name": "04-nep17-token",
  "groups": [],
  "features": {},
  "supportedstandards": ["NEP-17"],
  "abi": {
    "methods": [
      {
        "name": "symbol",
        "parameters": [],
        "returntype": "String",
        "offset": 0,
        "safe": true
      },
      {
        "name": "decimals",
        "parameters": [],
        "returntype": "Integer",
        "offset": 10,
        "safe": true
      },
      {
        "name": "transfer",
        "parameters": [
          {"name": "from", "type": "Hash160"},
          {"name": "to", "type": "Hash160"},
          {"name": "amount", "type": "Integer"},
          {"name": "data", "type": "Any"}
        ],
        "returntype": "Boolean",
        "offset": 20,
        "safe": false
      }
    ],
    "events": [
      {
        "name": "Transfer",
        "parameters": [
          {"name": "from", "type": "Hash160"},
          {"name": "to", "type": "Hash160"},
          {"name": "amount", "type": "Integer"}
        ]
      }
    ]
  },
  "permissions": [
    {"contract": "*", "methods": "*"}
  ],
  "trusts": [],
  "extra": {
    "Author": "EpicChain Rust Framework",
    "Description": "NEP-17 fungible token implementation",
    "Version": "1.0.0"
  }
}
```

## Best Practices

For optimal manifest generation:

1. **Use Contract Attributes** - Add metadata using `#[contract_author]`, `#[contract_version]`, etc.
2. **Document Methods** - Add comprehensive doc comments to all public methods
3. **Mark Safe Methods** - Use `#[safe]` attribute for read-only methods
4. **Follow Naming Conventions** - Use standard method names for token contracts
5. **Review Generated Manifest** - Always check the generated manifest before deployment

## Troubleshooting

### Common Issues

**Manifest generation fails:**
- Ensure WASM file exists and is valid
- Check that epicchain-wasm compiler is available
- Verify source file path is correct

**Missing method documentation:**
- Add doc comments to Rust methods
- Use `--source-code` parameter with epicchain-wasm
- Check that source file is accessible

**Incorrect method safety:**
- Use `#[safe]` attribute for read-only methods
- Follow naming conventions (get_, query_, etc.)
- Review generated ABI for accuracy

## Related Documentation

- **[Understanding EpicChain Manifests](understanding-epicchain-manifests.md)** - Manifest structure details
- **[Contract Attributes](contract-attributes.md)** - Metadata and permission attributes
- **[Code Documentation Style](code-documentation-style.md)** - Documentation standards