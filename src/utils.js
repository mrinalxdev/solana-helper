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

  encodeBase58(data) {
    return Buffer.from(data).toString("base64");
  }

  decodeBase58(data) {
    return Buffer.from(data, "base64").toString();
  }
}

modules.export = UtilsModule;
