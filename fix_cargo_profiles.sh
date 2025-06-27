#!/bin/bash

# Fix Cargo.toml profile sections in all examples
# This removes the profile.release section from individual packages
# to fix workspace configuration warnings

echo "🔧 Fixing Cargo.toml profile sections in all examples..."

# List of all example directories
examples=(
    "examples/02-simple-storage"
    "examples/03-counter"
    "examples/05-nep11-nft"
    "examples/06-nep24-royalty-nft"
    "examples/07-crowdfunding"
    "examples/08-staking"
    "examples/09-simple-dex"
    "examples/10-multisig-wallet"
    "examples/11-governance"
    "examples/12-oracle-price-feed"
    "examples/13-nft-marketplace"
)

for example in "${examples[@]}"; do
    cargo_file="$example/Cargo.toml"
    if [ -f "$cargo_file" ]; then
        echo "  📝 Fixing $cargo_file"
        
        # Create a temporary file without the profile section
        awk '
        /^\[profile\.release\]/ { skip = 1; next }
        /^\[/ && !/^\[profile\.release\]/ { skip = 0 }
        !skip { print }
        ' "$cargo_file" > "$cargo_file.tmp"
        
        # Replace the original file
        mv "$cargo_file.tmp" "$cargo_file"
        
        echo "    ✅ Fixed $cargo_file"
    else
        echo "    ⚠️  $cargo_file not found"
    fi
done

echo "✅ All Cargo.toml files fixed!"
echo ""
echo "📋 Summary:"
echo "   - Removed [profile.release] sections from individual packages"
echo "   - Profile configuration is now managed at workspace root"
echo "   - This eliminates workspace configuration warnings" 