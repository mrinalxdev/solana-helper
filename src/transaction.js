import { Transaction, SystemProgram } from "@solana/web3.js";

class TransactionModule {
  async sendTransaction(senderAccount, recieverAccount, amount) {
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: senderAccount.publicKey,
        toPubkey: recieverAccount.publicKey,
        lamports: amount,
      }),
    );

    const signature = await this.connection.sendTransaction(transaction, [
      senderAccount,
    ]);
    return signature;
  }

  async getTransactionDetails(signature) {
    const details = await this.connection.getTransaction(signature);
    return details;
  }
}

export default TransactionModule;
