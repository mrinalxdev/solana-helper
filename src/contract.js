import { Transaction, SystemProgram } from "@solana/web3.js";

class ContractModule {
  async deployContract(programData, senderAccount) {
    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: senderAccount.publicKey,
        newAccountPubkey: programData.programId,
        lamports: await this.connection.getMinimuBalanceForRentExemption(
          programData.data.length,
        ),
        space: programData.data.length,
        programId: programData.programId,
      }),
    );

    const signature = await this.connection.sendTransaction(transaction, [
      senderAccount,
    ]);
    return signature;
  }
}

export default ContractModule;
