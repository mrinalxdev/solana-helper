# Solana-Helper

> v0.5
> This is a package to do all the things with Solana blockchain development out of the box!
> Here is the list of the features in the current version

```bash

npm install solana-helper
```

```js
const SolanaHelper = require("solana-helper");
```

- Initialization
- Account Management
- Transaction Handling
- Smart Contract interactions Utility functions

with these Developers can build Solana application features like network switching, airdrop request, transaction simulation, program deployment etc.
This packaage easy-to-use,

with these Developers can build Solana application features like network switching, airdrop request, transaction simulation, program deployment etc.
This packaage easy-to-use, simple interface to work with the Solana blockchain, enabling developers to implementmore reliable and scalable dApps.simple interface to work with the Solana blockchain, enabling developers to implementmore reliable and scalable dApps.

## Initialization

```js
const solana = new SolanaHelper("https://api.mainnet-beta.solana.com");
```

Connect to the Solana Blockchain network and Switch between different network endpoints dynamically.

## Account Management

1. Creating a Account from secret Key

```js

const secretKey = new Uint8Array([...]); // Your Secret Key
const restoredAccount = solana.createAccountfrom

```
