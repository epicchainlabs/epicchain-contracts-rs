# Understanding and Customizing EpicChain Contract Manifests

This guide explains the structure of EpicChain contract manifests and how to customize them when needed.

## Manifest Structure

A EpicChain contract manifest is a JSON file that contains metadata about the contract. Here's an example of the structure:

```json
{
  "name": "MyContract",
  "groups": [],
  "features": {},
  "supportedstandards": ["NEP-17"],
  "abi": {
    "methods": [
      {
        "name": "transfer",
        "parameters": [
          {
            "name": "from",
            "type": "Hash160"
          },
          {
            "name": "to",
            "type": "Hash160"
          },
          {
            "name": "amount",
            "type": "Integer"
          },
          {
            "name": "data",
            "type": "Any"
          }
        ],
        "returntype": "Boolean",
        "safe": false
      }
    ],
    "events": []
  },
  "permissions": [
    {
      "contract": "*",
      "methods": ["*"]
    }
  ],
  "trusts": [],
  "extra": {
    "Description": "This is my NEP-17 token contract",
    "Author": "xmoohad",
    "MethodDescriptions": {
      "transfer": "Transfers tokens from one account to another"
    }
  }
}
```

## Key Components

### Basic Metadata

- **name**: The name of the contract, typically derived from the WASM filename
- **groups**: Signature groups for permission control (usually empty for simple contracts)
- **features**: Specialized contract features (usually empty)

### Standards and Interface

- **supportedstandards**: List of NEP standards implemented (e.g., ["NEP-17"], ["NEP-11"])
- **abi**: Application Binary Interface defining the contract's public methods
  - **methods**: List of exported methods with parameters and return types
  - **events**: List of events the contract can emit

### Security Settings

- **permissions**: What contracts and methods this contract is allowed to call
- **trusts**: Other contracts this contract implicitly trusts

### Additional Information

- **extra**: Custom fields with additional metadata
  - **Description**: Human-readable description of the contract
  - **Author**: Contract author information
  - **MethodDescriptions**: Documentation for individual methods

## Method Properties

Each method in the ABI has the following properties:

- **name**: The method name exactly as exported by the contract
- **parameters**: List of parameters with name and type
- **returntype**: The data type returned by the method
- **safe**: Whether the method is read-only (`true`) or modifies state (`false`)

## Parameter Types

EpicChain contracts use these standard parameter types:

- **Hash160**: 20-byte address or script hash
- **Hash256**: 32-byte hash (like a transaction ID)
- **ByteArray**: Variable-length array of bytes
- **String**: UTF-8 encoded string
- **Integer**: Number value
- **Boolean**: True/false value
- **Array**: A collection of values
- **Map**: Key-value pairs
- **Any**: Any data type
- **Void**: No value (for return types)

## Customizing the Generated Manifest

While our manifest generator creates a comprehensive manifest automatically, you may sometimes need to customize it:

### When to Customize

1. To add more detailed descriptions
2. To specify custom permissions
3. To manually declare support for standards
4. To add trust relationships with other contracts
5. To specify contract groups for multi-signature scenarios

### How to Customize

After generating the manifest:

1. Open the `.manifest.json` file in a text editor
2. Make your changes while maintaining valid JSON format
3. Ensure the manifest still adheres to the EpicChain blockchain requirements

### Permissions

The `permissions` field is particularly important for security. By default, our generator uses a permissive setting:

```json
"permissions": [
  {
    "contract": "*",
    "methods": ["*"]
  }
]
```

This allows your contract to call any method on any other contract. For a more secure approach, restrict to only the contracts and methods your contract needs:

```json
"permissions": [
  {
    "contract": "0xd2a4cff31913016155e38e474a2c06d08be276cf",
    "methods": ["transfer", "balanceOf"]
  },
  {
    "contract": "EpicPulse",
    "methods": ["transfer"]
  }
]
```

### Adding Events

If your contract emits events, you may need to manually add them to the `events` section:

```json
"events": [
  {
    "name": "Transfer",
    "parameters": [
      {
        "name": "from",
        "type": "Hash160"
      },
      {
        "name": "to",
        "type": "Hash160"
      },
      {
        "name": "amount",
        "type": "Integer"
      }
    ]
  }
]
```

## Best Practices

1. **Review Before Deployment**: Always review the generated manifest before deploying to the blockchain
2. **Minimal Permissions**: Only grant permissions to methods your contract actually needs
3. **Clear Descriptions**: Provide clear and detailed descriptions for your contract and methods
4. **Standard Compliance**: If implementing a standard, ensure all required methods are included
5. **Version Information**: Consider adding version information in the `extra` field

By following these guidelines, you'll create well-documented and secure EpicChain contract manifests that provide clear interfaces for users and other contracts.