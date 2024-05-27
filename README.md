# Solana-Helper

> v0.5
> This is a package to do all the things with Solana blockchain development out of the box!
> Here is the list of the features in the current version

```bash
## Terminal
npm install solana-helper
```

```js
// index.js
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

We have integrated handful features just like creating new accounts, restore accounts from secret keys
Fetching account details including balance and account info, requesting SOL sirdrops to accounts and fetching account details for multiple accounts.

1. Creating a new Account
```js
const newAccount = solana.createAccount();
```

2. Restoring an account from a secret key.
```js
const restoredAccount = solana.createAccountFromSecretKey(secretKey);
```

3. Fetching account balance
```js
solana.getAccountBalance(account.publicKey).then(balance => console.log(balance));
```
4. Requesting airdrop to an account
```js
solana.requestAirdrop(account.publicKey, 1000000).then(signature => console.log(signature));
```
5. Fetching account details for multiple accounts
```js
solana.getMultipleAccountInfo([account1.publicKey, account2.publicKey]).then(accountInfos => console.log(accountInfos));
```

## Transaction Handling
Transaction handling in "solana-helper" includes sending transactions between accounts, getting transaction details, simulating transactions without sending them, and retrieving recent account transactions. These features facilitate efficient management and monitoring of transactions on the Solana blockchain.

1. Sending a transaction between accounts
```js
solana.sendTransaction(senderAccount, receiverAccount, 1000000).then(signature => console.log(signature));
```

2. Getting transaction details
```js
solana.getTransactionDetails(transactionSignature).then(details => console.log(details));
```

3. Simulating a transaction without sending it.

```js
solana.simulateTransaction(senderAccount, receiverAccount, 1000000).then(result => console.log(result));
```

4. Retrieving recent transaction for an account

```js
solana.getRecentTransactions(account.publicKey).then(transactions => console.log(transactions));
```
## Smart Contract Interactions
This package simplifies smart contract interactions by facilitating the deployment of programs with multiple instructions, calling methods on contracts with parameters, and retrieving program account information. This streamlines the development process and enhances efficiency when interacting with Solana smart contracts.

1. Deploying a program with multiple instructions

```js
solana.deployProgramWithInstructions(programInstructions, senderAccount).then(signature => console.log(signature));
```

2. Calling a method on a smart contract with parameters.

```js
solana.invokeContractMethod(contractProgramId, methodName, methodArguments, senderAccount).then(signature => console.log(signature));
```