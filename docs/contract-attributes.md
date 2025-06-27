# Contract Attributes

EpicChain smart contracts require a **manifest file** that contains metadata about the contract, including its name, supported standards, permissions, and other information. The **EpicChain Rust Smart Contract Framework** provides contract attributes to specify this metadata directly in your Rust code, which are then automatically included in the generated manifest.

## Available Attributes

The following attributes are available for use in your EpicChain smart contracts:

### `#[contract_author]`

Specifies the author of the contract. This information is included in the manifest's `extra` section.

```rust
#[contract_author("EpicChain Rust Team")]
pub struct MyContract {
    // Contract state...
}
```

### `#[contract_version]`

Specifies the version of the contract. This information is included in the manifest's `extra` section.

```rust
#[contract_version("1.0.0")]
pub struct MyContract {
    // Contract state...
}
```

### `#[contract_standards]`

Declares the standards that the contract implements. This information is included in the manifest's `supportedstandards` section.

```rust
#[contract_standards("NEP-17")]
pub struct MyContract {
    // Contract state...
}
```

You can specify multiple standards by separating them with commas:

```rust
#[contract_standards("NEP-17, NEP-11")]
pub struct MyContract {
    // Contract state...
}
```

### `#[contract_permission]`

Defines the permissions that the contract requires. This information is included in the manifest's `permissions` section.

```rust
#[contract_permission("*:*")]
pub struct MyContract {
    // Contract state...
}
```

You can specify multiple permissions by using multiple attributes:

```rust
#[contract_permission("0x1234567890abcdef1234567890abcdef12345678:transfer")]
#[contract_permission("0xabcdef1234567890abcdef1234567890abcdef12:balanceOf")]
pub struct MyContract {
    // Contract state...
}
```

### `#[contract_meta]`

Adds additional metadata to the contract manifest. This information is included in the manifest's `extra` section.

```rust
#[contract_meta("Email", "devs@epic-chain.org")]
#[contract_meta("Website", "https://epic-chain.org")]
pub struct MyContract {
    // Contract state...
}
```

## Method Attributes

In addition to contract-level attributes, you can also use method-level attributes to specify information about contract methods:

### `#[method]`

Marks a method as a contract method that should be included in the contract's ABI.

```rust
#[method]
pub fn transfer(&self, from: H160, to: H160, amount: Int256) -> bool {
    // Method implementation...
}
```

### `#[safe]`

Marks a method as read-only (does not modify state). This information is included in the method's `safe` property in the ABI.

```rust
#[method]
#[safe]
pub fn balance_of(&self, account: H160) -> Int256 {
    // Method implementation...
}
```

## Example

Here's a complete example of a contract that uses all the available attributes:

```rust
use epicchain_contract::prelude::*;

#[contract_author("EpicChain Rust Team")]
#[contract_version("1.0.0")]
#[contract_standards("NEP-17")]
#[contract_permission("*:*")]
#[contract_meta("Email", "devs@epic-chain.org")]
#[contract_meta("Website", "https://epic-chain.org")]
pub struct MyToken {
    name: StorageItem<ByteString>,
    symbol: StorageItem<ByteString>,
    decimals: StorageItem<u8>,
    total_supply: StorageItem<Int256>,
    balances: StorageMap<H160, Int256>,
}

#[contract_impl]
impl MyToken {
    pub fn init() -> Self {
        // Initialization code...
    }

    #[method]
    #[safe]
    pub fn symbol(&self) -> ByteString {
        // Method implementation...
    }

    #[method]
    #[safe]
    pub fn decimals(&self) -> u8 {
        // Method implementation...
    }

    #[method]
    #[safe]
    pub fn total_supply(&self) -> Int256 {
        // Method implementation...
    }

    #[method]
    #[safe]
    pub fn balance_of(&self, account: H160) -> Int256 {
        // Method implementation...
    }

    #[method]
    pub fn transfer(&self, from: H160, to: H160, amount: Int256) -> bool {
        // Method implementation...
    }
}
```

## Manifest Generation

When you build your contract using the framework's build system, the contract attributes are automatically processed and included in the generated manifest file:

```bash
# Build contract with attributes included in manifest
cd examples/04-nep17-token
make                    # Build WASM → NEF + Manifest with attributes
make manifest           # Generate manifest using epicchain-wasm compiler
```

The **epicchain-wasm compiler** processes the contract attributes and includes them in the corresponding sections of the manifest file.

For example, the contract above would generate a manifest with the following sections:

```json
{
  "name": "MyToken",
  "groups": [],
  "supportedstandards": ["NEP-17"],
  "abi": {
    "methods": [
      {
        "name": "symbol",
        "parameters": [],
        "returntype": "ByteString",
        "offset": 0,
        "safe": true
      },
      {
        "name": "decimals",
        "parameters": [],
        "returntype": "Integer",
        "offset": 0,
        "safe": true
      },
      {
        "name": "total_supply",
        "parameters": [],
        "returntype": "Integer",
        "offset": 0,
        "safe": true
      },
      {
        "name": "balance_of",
        "parameters": [
          {
            "name": "account",
            "type": "Hash160"
          }
        ],
        "returntype": "Integer",
        "offset": 0,
        "safe": true
      },
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
          }
        ],
        "returntype": "Boolean",
        "offset": 0,
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
    "Author": "EpicChain Rust Team",
    "Version": "1.0.0",
    "Email": "devs@epic-chain.org",
    "Website": "https://epic-chain.org"
  }
}
```

## Attribute Macros

The contract attributes in epicchain-contract-rs are implemented as attribute macros, which are a feature of Rust's procedural macro system. These macros are applied to items in your code using the `#[...]` syntax.

It's important to note that these are not annotations in doc comments, but actual Rust attribute macros that are processed during compilation. The compiler uses these attributes to generate the appropriate manifest information.

For example, this is the correct way to use the contract attributes:

```rust
#[contract_author("EpicChain Rust Team")]
#[contract_version("1.0.0")]
#[contract_standards("NEP-17")]
#[contract_permission("*:*")]
#[contract_meta("Email", "devs@epic-chain.org")]
#[contract_meta("Website", "https://epic-chain.org")]
pub struct MyToken {
    // Contract state...
}
```

Each attribute macro takes one or more string literals as arguments, which are then used to generate the corresponding sections in the manifest file.

## Working Examples

The framework includes several examples that demonstrate contract attributes in action:

### NEP-17 Token Example
```bash
cd examples/04-nep17-token
cat src/lib.rs  # See contract attributes in use
make            # Build and see generated manifest
cat build/04-nep17-token.manifest.json  # View generated manifest
```

### NFT Example
```bash
cd examples/05-nep11-nft
cat src/lib.rs  # See NFT contract attributes
make            # Build with NEP-11 attributes
```

### Governance Example
```bash
cd examples/11-governance
cat src/lib.rs  # See governance contract attributes
make            # Build with custom metadata
```

## Best Practices

1. **Always Include Author and Version** - Use `#[contract_author]` and `#[contract_version]`
2. **Declare Standards** - Use `#[contract_standards]` for NEP compliance
3. **Set Appropriate Permissions** - Use `#[contract_permission]` for security
4. **Add Useful Metadata** - Use `#[contract_meta]` for additional information
5. **Mark Safe Methods** - Use `#[safe]` for read-only methods

## Related Documentation

- **[Manifest Generation](manifest-generation.md)** - How manifests are generated
- **[Understanding EpicChain Manifests](understanding-epicchain-manifests.md)** - Manifest structure
- **[Getting Started Guide](getting-started.md)** - Complete setup and examples
