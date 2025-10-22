#!/bin/bash
# Linera environment variables for local testnet
# Usage: source scripts/env.sh

export LINERA_WALLET="/var/folders/29/d1wmh7lx1pjc6nn6pstplg600000gn/T/.tmpU0fpON/wallet_0.json"
export LINERA_KEYSTORE="/var/folders/29/d1wmh7lx1pjc6nn6pstplg600000gn/T/.tmpU0fpON/keystore_0.json"
export LINERA_STORAGE="rocksdb:/var/folders/29/d1wmh7lx1pjc6nn6pstplg600000gn/T/.tmpU0fpON/client_0.db"

echo "✅ Linera environment variables set!"
echo "Wallet: $LINERA_WALLET"
