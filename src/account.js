import { Keypair, PublicKey } from "@solana/web3.js";

class AccountModule {
  createAccount() {
    const account = Keypair.generate();
    return account;
  }

  createAccountFromSecretKey(secretKey) {
    const account = Keypair.fromSecretKey(secretKey);
    return account;
  }

  async getAccountInfo(publicKey) {
    const pubk = new PublicKey(publicKey);
    return await this.connection.getAccountInfo(pubk);
  }

  async getAccountBalance(publicKey) {
    const pubkey = new PublicKey(publicKey);
    const balance = await this.connection.getBalance(pubkey);
    return balance;
  }

  async requestAirdrop(publicKey, amount) {
    const pubkey = new PublicKey(publicKey);
    const signature = await this.connection.requestAirdrop(pubkey, amount);
    return signature;
  }
}

export default AccountModule;
