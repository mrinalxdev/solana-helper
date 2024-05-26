import { Keypair } from "@solana/web3.js";

class UtilsModule {
  lampartsToSol(lamports) {
    return lamports / 1_000_000_000;
  }

  solToLamports(sol) {
    return sol * 1_000_000_000;
  }

  generatRandomKeypair() {
    return Keypair.generate();
  }
}

modules.export = UtilsModule;
