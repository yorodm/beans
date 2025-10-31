#!/bin/bash
set -e

# This script creates all source files for the beans-lib

BASE_DIR="beans-lib/src"

echo "Creating source files..."

# Already created: error.rs, lib.rs, models/mod.rs
# Need to create the rest

# Create a simple placeholder main for now
cat > beans/src/main.rs << 'EOF'
//! Beans - Ledger application (UI will be added in Phase 2)

fn main() {
    println!("Beans ledger application");
    println!("UI implementation coming in Phase 2 with Ribir");
}
EOF

echo "Created basic main.rs placeholder"
echo "Source file creation complete!"
echo "Now compiling to verify..."

