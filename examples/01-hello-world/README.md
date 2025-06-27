# Hello World Smart Contract

A comprehensive introduction to EpicChain smart contract development using Rust. This example demonstrates fundamental concepts and best practices for building secure, efficient smart contracts.

## 🎯 **Learning Objectives**

After studying this example, you will understand:

- ✅ Basic contract structure and organization
- ✅ Contract attributes and metadata
- ✅ Safe vs unsafe method distinctions
- ✅ Storage operations and data persistence
- ✅ Event emission and logging
- ✅ Witness checking and authorization
- ✅ Input validation and error handling
- ✅ Testing smart contracts

## 🏗 **Contract Features**

### **Core Functionality**
- **Greeting Management**: Set and retrieve greeting messages
- **Visitor Tracking**: Register visitors and track visit count
- **Information Display**: Get contract metadata and statistics
- **State Management**: Reset contract state (owner only)

### **Security Features**
- **Authorization**: Owner-only methods with witness checking
- **Input Validation**: Length and content validation for all inputs
- **Error Handling**: Comprehensive error messages and logging
- **Safe Methods**: Read-only methods marked as safe

### **Storage Pattern**
- **Efficient Keys**: Structured storage key naming
- **Data Types**: Proper serialization of different data types
- **State Persistence**: Reliable data storage and retrieval

## 📋 **Contract Methods**

### **Safe Methods** (Read-only, no state changes)

#### `get_greeting() -> ByteString`
Returns the current greeting message. If no custom greeting is set, returns the default "Hello, EpicChain World!".

#### `get_visitor_count() -> Int256`
Returns the total number of registered visitors.

#### `get_visitor(visitor_number: Int256) -> ByteString`
Retrieves visitor information by their registration number.

#### `get_info() -> Map<ByteString, Any>`
Returns comprehensive contract information including:
- Contract name and version
- Author information
- Current visitor count
- Current greeting message

### **Unsafe Methods** (Can modify state)

#### `set_greeting(new_greeting: ByteString) -> bool`
**Authorization**: Contract owner only
**Purpose**: Updates the greeting message
**Validation**: 1-100 characters required
**Events**: Emits `GreetingChanged` event

#### `say_hello(visitor_name: ByteString) -> ByteString`
**Authorization**: Public (anyone can call)
**Purpose**: Registers a new visitor and returns personalized greeting
**Validation**: 1-50 characters required
**Events**: Emits `VisitorRegistered` event

#### `reset() -> bool`
**Authorization**: Contract owner only
**Purpose**: Resets all contract state to initial values
**Events**: Emits `ContractReset` event

## 🔧 **Building and Testing**

### **Prerequisites**
```bash
# Install Rust with WASM target
rustup target add wasm32-unknown-unknown

# Ensure you're in the project root
cd epicchain-contract-rs
```

### **Build the Contract**
```bash
cd examples-new/01-hello-world

# Build optimized WASM
cargo build --target wasm32-unknown-unknown --release

# Or use the Makefile
make build
```

### **Run Tests**
```bash
# Run unit tests
cargo test

# Run with output
cargo test -- --nocapture
```

### **Deploy to Testnet**
```bash
# Compile to NEF format (requires epicchain-wasm compiler)
make deploy
```

## 📖 **Code Walkthrough**

### **1. Contract Structure**
```rust
#[contract_author("EpicChain Rust Framework", "devs@epic-chain.org")]
#[contract_version("1.0.0")]
#[contract_standards("")]
#[contract_permission("*", "*")]
pub struct HelloWorld {
    greeting_key: ByteString,
    counter_key: ByteString,
    visitors_prefix: ByteString,
}
```

**Key Points:**
- Contract attributes provide metadata for the manifest
- Storage keys are defined as struct fields for organization
- Permissions allow any contract to call any method

### **2. Safe Method Example**
```rust
#[method]
#[safe]
pub fn get_greeting(&self) -> ByteString {
    let storage = Storage::get_read_only_context();
    match Storage::get(storage, self.greeting_key.clone()) {
        Some(greeting) => greeting,
        None => ByteString::from_literal("Hello, EpicChain World!"),
    }
}
```

**Key Points:**
- `#[safe]` attribute marks read-only methods
- Uses read-only storage context for efficiency
- Provides default value when no data exists

### **3. Authorization Pattern**
```rust
let contract_hash = Runtime::get_executing_script_hash();
if !Runtime::check_witness(contract_hash) {
    Runtime::log(ByteString::from_literal("Unauthorized"));
    return false;
}
```

**Key Points:**
- Uses contract hash as owner identifier
- `check_witness` verifies caller authorization
- Returns early with error message on failure

### **4. Event Emission**
```rust
Runtime::notify(
    ByteString::from_literal("VisitorRegistered"), 
    visitor_name.into_any()
);
```

**Key Points:**
- Events provide external visibility into contract actions
- Use descriptive event names
- Include relevant data in event payload

## 🛡 **Security Considerations**

### **Input Validation**
- All user inputs are validated for length and content
- Empty inputs are rejected with clear error messages
- Maximum lengths prevent storage abuse

### **Authorization**
- Owner-only methods use witness checking
- Contract hash serves as owner identifier
- Unauthorized access attempts are logged

### **Error Handling**
- All error conditions return meaningful messages
- Failed operations don't leave contract in inconsistent state
- Logging provides debugging information

## 🚀 **Next Steps**

After mastering this example, proceed to:

1. **[02-simple-storage](../02-simple-storage/)** - Advanced storage patterns
2. **[03-counter](../03-counter/)** - State management techniques
3. **[04-nep17-token](../04-nep17-token/)** - Token standard implementation

## 💡 **Best Practices Demonstrated**

- ✅ **Clear Documentation**: Comprehensive comments and documentation
- ✅ **Structured Code**: Logical organization and naming conventions
- ✅ **Error Handling**: Robust error checking and user feedback
- ✅ **Security First**: Authorization and input validation
- ✅ **Testing**: Unit tests for core functionality
- ✅ **Events**: Proper event emission for external monitoring
- ✅ **Gas Efficiency**: Optimized storage operations and minimal allocations
