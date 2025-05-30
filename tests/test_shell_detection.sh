#!/bin/bash

# Test script for shell mangling detection

echo "🧪 Testing Shell Mangling Detection"
echo "====================================="
echo

echo "Testing patterns that should trigger interactive mode:"
echo

echo "1. Testing 'what s the difference' (simulates what's):"
cargo run --quiet -- what s the difference < /dev/null

echo
echo "2. Testing 'don t know' (simulates don't):"
cargo run --quiet -- don t know < /dev/null

echo
echo "3. Testing 'you re right' (simulates you're):"
cargo run --quiet -- you re right < /dev/null

echo
echo "4. Testing input ending with ' s' (simulates what's):"
cargo run --quiet -- what s < /dev/null

echo
echo "5. Testing normal input (should NOT trigger):"
cargo run --quiet -- "what is rust" < /dev/null

echo
echo "6. Testing quoted input (should NOT trigger):"
cargo run --quiet -- "what's the difference between rust and go" < /dev/null
