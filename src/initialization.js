import { Connection } from "@solana/web3.js";

class SolanaHelper {
  constructor(endpoint) {
    this.connection = new Connection(endpoint);
  }

  switchNetwork(newEndpoint) {
    this.connection = new Connection(newEndpoint);
    console.log(`Switch to new network: ${newEndpoint}`);
  }
}

export default SolanaHelper;
