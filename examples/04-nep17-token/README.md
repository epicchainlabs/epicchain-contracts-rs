# NEP-17 Fungible Token Contract

A complete, production-ready implementation of the NEP-17 fungible token standard for Neo N3. This contract demonstrates advanced token mechanics, security patterns, and administrative controls suitable for real-world deployment.

## 🎯 **Key Features**

### **Core NEP-17 Compliance**
- ✅ **symbol()** - Token symbol identification
- ✅ **decimals()** - Token precision (up to 18 decimals)
- ✅ **totalSupply()** - Total token supply tracking
- ✅ **balanceOf()** - Account balance queries
- ✅ **transfer()** - Secure token transfers with witness verification

### **Advanced Token Features**
- 🔐 **Allowance System** - Delegated transfers with approve/transferFrom
- 🏭 **Minting & Burning** - Controlled token supply management
- ⏸️ **Pausable Operations** - Emergency pause functionality
- 👑 **Administrative Controls** - Owner and minter management
- 🛡️ **Security Features** - Overflow protection and input validation

### **Production-Ready Elements**
- 📊 **Comprehensive Events** - Transfer, Approval, Mint, Burn events
- 🔍 **Detailed Logging** - Operation tracking and error reporting
- ⛽ **EpicPulse Optimization** - Efficient storage patterns
- 🧪 **Extensive Testing** - Unit tests for all functionality

## 📋 **Contract Methods**

### **NEP-17 Required Methods**

#### `symbol() -> ByteString`
Returns the token symbol (e.g., "MYTOKEN").

#### `decimals() -> u32`
Returns the number of decimal places (0-18).

#### `total_supply() -> Int256`
Returns the total token supply.

#### `balance_of(account: H160) -> Int256`
Returns the token balance of the specified account.

#### `transfer(from: H160, to: H160, amount: Int256, data: Any) -> bool`
Transfers tokens from one account to another with witness verification.

### **Extended Functionality**

#### `allowance(owner: H160, spender: H160) -> Int256`
Returns the amount that spender is allowed to transfer on behalf of owner.

#### `approve(owner: H160, spender: H160, amount: Int256) -> bool`
Approves spender to transfer up to amount tokens on behalf of owner.

#### `transfer_from(spender: H160, from: H160, to: H160, amount: Int256, data: Any) -> bool`
Transfers tokens using allowance mechanism.

#### `mint(to: H160, amount: Int256) -> bool`
Creates new tokens and assigns them to the specified account (authorized minters only).

#### `burn(from: H160, amount: Int256) -> bool`
Destroys tokens from the specified account (account owner only).

### **Administrative Methods**

#### `deploy(owner, symbol, decimals, initial_supply, max_supply) -> bool`
One-time contract deployment with initial parameters.

#### `add_minter(minter: H160) -> bool`
Adds an authorized minter (owner only).

#### `remove_minter(minter: H160) -> bool`
Removes an authorized minter (owner only).

#### `pause() -> bool`
Pauses all token transfers (owner only).

#### `unpause() -> bool`
Resumes token transfers (owner only).

## 🔧 **Usage Examples**

### **Basic Token Operations**

```rust
// Deploy token
let success = token.deploy(
    owner_address,
    ByteString::from_literal("MYTOKEN"),
    8, // decimals
    Int256::new(1_000_000 * 100_000_000), // 1M tokens with 8 decimals
    Int256::new(10_000_000 * 100_000_000)  // 10M max supply
);

// Transfer tokens
let success = token.transfer(
    sender_address,
    recipient_address,
    Int256::new(100 * 100_000_000), // 100 tokens
    Any::null()
);

// Check balance
let balance = token.balance_of(account_address);
```

### **Allowance System**

```rust
// Approve spending
let success = token.approve(
    owner_address,
    spender_address,
    Int256::new(50 * 100_000_000) // 50 tokens
);

// Transfer on behalf
let success = token.transfer_from(
    spender_address,
    owner_address,
    recipient_address,
    Int256::new(25 * 100_000_000), // 25 tokens
    Any::null()
);
```

### **Administrative Operations**

```rust
// Add minter
let success = token.add_minter(minter_address);

// Mint new tokens
let success = token.mint(
    recipient_address,
    Int256::new(1000 * 100_000_000) // 1000 tokens
);

// Pause contract
let success = token.pause();
```

## 🛡️ **Security Features**

### **Access Control**
- **Owner-only functions** protected by witness checking
- **Minter authorization** system for controlled token creation
- **Self-transfer protection** prevents unnecessary operations

### **Input Validation**
- **Negative amount protection** rejects invalid transfer amounts
- **Balance verification** ensures sufficient funds before transfers
- **Allowance checking** validates delegated transfer permissions

### **Overflow Protection**
- **Safe arithmetic** using checked operations
- **Supply limits** enforced through max_supply parameter
- **Balance consistency** maintained across all operations

### **Emergency Controls**
- **Pausable functionality** for emergency situations
- **Owner transfer** capabilities for contract management
- **Minter management** for supply control

## 📊 **Events**

### **Transfer Event**
```
Transfer(from: H160, to: H160, amount: Int256)
```
Emitted on every token transfer, including mints (from=null) and burns (to=null).

### **Approval Event**
```
Approval(owner: H160, spender: H160, amount: Int256)
```
Emitted when allowance is set or modified.

### **Administrative Events**
- `TokenDeployed` - Contract deployment
- `TokensMinted` - New tokens created
- `TokensBurned` - Tokens destroyed
- `MinterAdded/Removed` - Minter authorization changes
- `ContractPaused/Unpaused` - Pause state changes

## 🧪 **Testing**

### **Build and Test**
```bash
# Build the contract
make build

# Run tests
make test

# Run tests with output
make test-verbose
```

### **Test Coverage**
- ✅ Token deployment and initialization
- ✅ Basic transfer operations
- ✅ Allowance system functionality
- ✅ Minting and burning operations
- ✅ Administrative controls
- ✅ Security validations
- ✅ Edge cases and error conditions

## 🚀 **Deployment Guide**

### **1. Prepare Parameters**
```rust
let owner = H160::from_string("0x1234..."); // Your address
let symbol = ByteString::from_literal("MYTOKEN");
let decimals = 8u32;
let initial_supply = Int256::new(1_000_000 * 100_000_000);
let max_supply = Int256::new(10_000_000 * 100_000_000);
```

### **2. Deploy Contract**
```bash
# Compile to NEF
make compile

# Deploy using neo-cli
neo> deploy path/to/contract.nef path/to/manifest.json
```

### **3. Initialize Token**
```bash
# Call deploy method
neo> invokefunction <contract_hash> deploy <owner> <symbol> <decimals> <initial_supply> <max_supply>
```

## 💡 **Best Practices**

### **For Token Creators**
- Set reasonable max_supply limits
- Use appropriate decimal places (8 for most use cases)
- Implement proper minter management
- Monitor contract events for unusual activity

### **For Integrators**
- Always check return values from contract calls
- Implement proper error handling
- Use events for transaction monitoring
- Validate token contract before integration

### **For Users**
- Verify contract authenticity before use
- Check allowances before delegated transfers
- Monitor balance changes through events
- Be aware of pause functionality

## 🔗 **Related Examples**

- **[05-nep11-nft](../05-nep11-nft/)** - Non-fungible token implementation
- **[06-nep24-royalty-nft](../06-nep24-royalty-nft/)** - NFT with royalty features
- **[08-staking](../08-staking/)** - Token staking contract
- **[09-dex](../09-dex/)** - Decentralized exchange using tokens
