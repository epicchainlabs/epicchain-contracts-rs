# Custom Serialization Guide for epicchain-contract-rs

This guide explains how to implement custom serialization and deserialization for complex data structures in EpicChain smart contracts using the epicchain-contract-rs framework.

## Introduction

EpicChain smart contracts often need to store complex data structures in the blockchain's key-value storage. Since the storage only accepts byte representations, we need serialization and deserialization strategies to convert between Rust types and byte representations.

## Built-in Serialization

The epicchain-contract-rs framework provides built-in serialization for common types:

- Basic types (integers, booleans) 
- ByteString
- H160 (EpicChain addresses)
- Int256 (large integers)

However, for more complex types like:

- Collections (arrays, maps)
- Custom structs
- Nested data structures

You'll need either the `#[epicchain::structs]` macro or custom serialization logic.

## Using the `#[epicchain::structs]` Macro

For simple struct serialization, use the provided macro:

```rust
#[epicchain::structs]
struct TokenMetadata {
    name: ByteString,
    description: ByteString,
    creator: H160,
    created_at: u64,
    total_supply: Int256,
}

// Usage
let metadata = TokenMetadata {
    name: ByteString::from("MyToken"),
    description: ByteString::from("A test token"),
    creator: H160::from_bytes(&[1; 20]),
    created_at: 1234567890,
    total_supply: Int256::from_i32(1000),
};

// Serialize
let serialized = metadata.serialize();

// Deserialize
let deserialized = TokenMetadata::deserialize(serialized).unwrap();
```

## Manual Serialization

For advanced use cases or specialized formats, you may need to implement serialization manually.

### Serializing Collections

#### Array Serialization Example

```rust
/// Serialize an array of ByteString
fn serialize_tokens(tokens: Array<ByteString>) -> ByteString {
    let mut result = ByteString::empty();
    
    // 1. Store array length (4 bytes)
    let len = tokens.len();
    let len_bytes = [
        ((len >> 24) & 0xFF) as u8,
        ((len >> 16) & 0xFF) as u8,
        ((len >> 8) & 0xFF) as u8,
        (len & 0xFF) as u8,
    ];
    result = result.concat(&ByteString::from_bytes(&len_bytes));
    
    // 2. Store each element
    for i in 0..tokens.len() {
        let token = tokens.get(i);
        
        // 2.1 Store element length (2 bytes)
        let token_len = token.len();
        let token_len_bytes = [
            ((token_len >> 8) & 0xFF) as u8,
            (token_len & 0xFF) as u8,
        ];
        result = result.concat(&ByteString::from_bytes(&token_len_bytes));
        
        // 2.2 Store element bytes
        result = result.concat(&token);
    }
    
    result
}

/// Deserialize an array of ByteString
fn deserialize_tokens(data: ByteString) -> Array<ByteString> {
    let mut result = Array::<ByteString>::new();
    let bytes = data.to_bytes();
    
    // Need at least 4 bytes for length
    if bytes.len() < 4 {
        return result;
    }
    
    // 1. Read array length
    let len = ((bytes[0] as usize) << 24) |
              ((bytes[1] as usize) << 16) |
              ((bytes[2] as usize) << 8) |
              (bytes[3] as usize);
    
    // 2. Read each element
    let mut pos = 4;
    for _ in 0..len {
        // Need at least 2 bytes for element length
        if pos + 2 > bytes.len() {
            break;
        }
        
        // 2.1 Read element length
        let token_len = ((bytes[pos] as usize) << 8) | (bytes[pos + 1] as usize);
        pos += 2;
        
        // 2.2 Read element bytes
        if pos + token_len > bytes.len() {
            break;
        }
        
        let token_bytes = &bytes[pos..pos + token_len];
        let token = ByteString::from_bytes(token_bytes);
        result.push(token);
        
        pos += token_len;
    }
    
    result
}
```

#### Map Serialization Example

```rust
/// Serialize a Map of ByteString to ByteString
fn serialize_properties(properties: Map<ByteString, ByteString>) -> ByteString {
    let mut result = ByteString::empty();
    let keys = properties.keys();
    
    // 1. Store map size (4 bytes)
    let len = keys.len();
    let len_bytes = [
        ((len >> 24) & 0xFF) as u8,
        ((len >> 16) & 0xFF) as u8,
        ((len >> 8) & 0xFF) as u8,
        (len & 0xFF) as u8,
    ];
    result = result.concat(&ByteString::from_bytes(&len_bytes));
    
    // 2. Store each key-value pair
    for i in 0..keys.len() {
        let key = keys.get(i);
        let value = properties.get(key.clone()).unwrap();
        
        // 2.1 Store key length and key
        let key_len = key.len();
        let key_len_bytes = [
            ((key_len >> 8) & 0xFF) as u8,
            (key_len & 0xFF) as u8,
        ];
        result = result.concat(&ByteString::from_bytes(&key_len_bytes));
        result = result.concat(&key);
        
        // 2.2 Store value length and value
        let value_len = value.len();
        let value_len_bytes = [
            ((value_len >> 8) & 0xFF) as u8,
            (value_len & 0xFF) as u8,
        ];
        result = result.concat(&ByteString::from_bytes(&value_len_bytes));
        result = result.concat(&value);
    }
    
    result
}

/// Deserialize a Map of ByteString to ByteString
fn deserialize_properties(data: ByteString) -> Map<ByteString, ByteString> {
    let mut result = Map::<ByteString, ByteString>::new();
    let bytes = data.to_bytes();
    
    // Need at least 4 bytes for size
    if bytes.len() < 4 {
        return result;
    }
    
    // 1. Read map size
    let len = ((bytes[0] as usize) << 24) |
              ((bytes[1] as usize) << 16) |
              ((bytes[2] as usize) << 8) |
              (bytes[3] as usize);
    
    // 2. Read each key-value pair
    let mut pos = 4;
    for _ in 0..len {
        // 2.1 Read key length and key
        if pos + 2 > bytes.len() {
            break;
        }
        
        let key_len = ((bytes[pos] as usize) << 8) | (bytes[pos + 1] as usize);
        pos += 2;
        
        if pos + key_len > bytes.len() {
            break;
        }
        
        let key_bytes = &bytes[pos..pos + key_len];
        let key = ByteString::from_bytes(key_bytes);
        pos += key_len;
        
        // 2.2 Read value length and value
        if pos + 2 > bytes.len() {
            break;
        }
        
        let value_len = ((bytes[pos] as usize) << 8) | (bytes[pos + 1] as usize);
        pos += 2;
        
        if pos + value_len > bytes.len() {
            break;
        }
        
        let value_bytes = &bytes[pos..pos + value_len];
        let value = ByteString::from_bytes(value_bytes);
        pos += value_len;
        
        // Insert key-value pair into map
        result.insert(key, value);
    }
    
    result
}
```

### Serializing Custom Structs Manually

For structs with more complex serialization needs:

```rust
struct NFTToken {
    id: ByteString,
    owner: H160,
    metadata: Map<ByteString, ByteString>,
    created_at: u64,
}

impl NFTToken {
    fn serialize(&self) -> ByteString {
        let mut result = ByteString::empty();
        
        // Serialize ID (length + bytes)
        let id_len = self.id.len();
        let id_len_bytes = [
            ((id_len >> 8) & 0xFF) as u8,
            (id_len & 0xFF) as u8,
        ];
        result = result.concat(&ByteString::from_bytes(&id_len_bytes));
        result = result.concat(&self.id);
        
        // Serialize Owner (fixed 20 bytes)
        result = result.concat(&ByteString::from_bytes(&self.owner.to_bytes()));
        
        // Serialize Metadata (using the map serialization from above)
        let serialized_metadata = serialize_properties(self.metadata.clone());
        result = result.concat(&serialized_metadata);
        
        // Serialize timestamp (8 bytes)
        let timestamp_bytes = [
            ((self.created_at >> 56) & 0xFF) as u8,
            ((self.created_at >> 48) & 0xFF) as u8,
            ((self.created_at >> 40) & 0xFF) as u8,
            ((self.created_at >> 32) & 0xFF) as u8,
            ((self.created_at >> 24) & 0xFF) as u8,
            ((self.created_at >> 16) & 0xFF) as u8,
            ((self.created_at >> 8) & 0xFF) as u8,
            (self.created_at & 0xFF) as u8,
        ];
        result = result.concat(&ByteString::from_bytes(&timestamp_bytes));
        
        result
    }
    
    fn deserialize(data: ByteString) -> Option<Self> {
        let bytes = data.to_bytes();
        let mut pos = 0;
        
        // Need at least 2 bytes for ID length
        if bytes.len() < 2 {
            return None;
        }
        
        // Deserialize ID
        let id_len = ((bytes[pos] as usize) << 8) | (bytes[pos + 1] as usize);
        pos += 2;
        
        if pos + id_len > bytes.len() {
            return None;
        }
        
        let id_bytes = &bytes[pos..pos + id_len];
        let id = ByteString::from_bytes(id_bytes);
        pos += id_len;
        
        // Deserialize Owner (needs 20 bytes)
        if pos + 20 > bytes.len() {
            return None;
        }
        
        let owner_bytes = &bytes[pos..pos + 20];
        let owner = H160::from_bytes(owner_bytes);
        pos += 20;
        
        // Deserialize Metadata
        // We need all remaining bytes except the final 8 for timestamp
        if pos + 8 >= bytes.len() {
            return None;
        }
        
        let metadata_bytes = &bytes[pos..bytes.len() - 8];
        let metadata = deserialize_properties(ByteString::from_bytes(metadata_bytes));
        pos = bytes.len() - 8;
        
        // Deserialize timestamp
        let created_at = ((bytes[pos] as u64) << 56) |
                         ((bytes[pos + 1] as u64) << 48) |
                         ((bytes[pos + 2] as u64) << 40) |
                         ((bytes[pos + 3] as u64) << 32) |
                         ((bytes[pos + 4] as u64) << 24) |
                         ((bytes[pos + 5] as u64) << 16) |
                         ((bytes[pos + 6] as u64) << 8) |
                         (bytes[pos + 7] as u64);
        
        Some(NFTToken {
            id,
            owner,
            metadata,
            created_at,
        })
    }
}
```

## Serialization Formats

There are several formats you can use for serialization:

### Length-Prefixed Format

Prefixing each variable-length field with its length (as shown in examples above).

**Pros:**
- Easy to implement
- Efficient for sequential access
- Clear boundaries between fields

**Cons:**
- Verbose for many small fields
- Not as compact as some other formats

### Tag-Length-Value (TLV) Format

Using a tag to identify the field, followed by length and value:

```rust
enum FieldTag {
    Id = 1,
    Owner = 2,
    Metadata = 3,
    CreatedAt = 4,
}

// Serialize with tag-length-value
let tag_bytes = [FieldTag::Id as u8];
result = result.concat(&ByteString::from_bytes(&tag_bytes));
// Then length and value...
```

**Pros:**
- Fields can be optional
- Order can be flexible
- Backward/forward compatible

**Cons:**
- More complex to implement
- Slightly less efficient

### Fixed-Width Format

For structs with fixed-size fields:

```rust
// All fields have fixed size
struct FixedStruct {
    field1: u32,  // 4 bytes
    field2: u64,  // 8 bytes
    address: H160, // 20 bytes
}
```

**Pros:**
- Very compact
- Fast serialization/deserialization

**Cons:**
- Inflexible
- Not suitable for variable-length data

## Best Practices

1. **Format Consistency**: Use the same serialization format throughout your contract
2. **Error Handling**: Always validate data during deserialization
3. **Versioning**: Consider including a version byte for format changes
4. **Gas Efficiency**: Balance between readability and gas efficiency 
5. **Testing**: Thoroughly test serialization with edge cases

## Using Existing Libraries

While epicchain-contract-rs doesn't have built-in serialization for all types, you can adapt techniques from common serialization formats:

- [Borsh](https://github.com/near/borsh) - Binary Object Representation Serializer for Hashing
- [Bincode](https://github.com/bincode-org/bincode) - Binary serialization for Rust
- [MessagePack](https://msgpack.org) - Efficient binary serialization format

Just be careful with dependencies in no_std environments.

## Conclusion

Choosing the right serialization strategy is critical for EpicChain contract development. The examples in this guide should help you implement efficient serialization for any complex data structure in your smart contracts. 