import { Transaction, SystemProgram, PublicKey } from "@solana/web3.js";

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

  async createTokenTrnasferTransaction(
    senderAccount,
    receiverPublicKey,
    amount,
    tokenProgramId,
  ) {
    const transaction = new Transaction().add(
      SystemProgram.tranfer({
        fromPubkey: senderAccount.publicKey,
        toPubkey: new PublicKey(receiverPublicKey),
        lamports: amount,
        programId: tokenProgramId,
      }),
    );

    const signature = await this.connection.sendTransaction(transaction, [
      senderAccount,
    ]);
    return signature;
  }
}

export default TransactionModule;
