import SolanaHelper from "./initialization";
import { AccountModule } from "./account";
import { TransactionModule } from "./transaction";
import { ContractModule } from "./contract";
import { UtilsModule } from "./utils";

class SolanaHelperExtended extends SolanaHelper {
  constructor(endpoint) {
    super(endpoint);
    Object.assign(this, AccountModule.prototype);
    Object.assign(this, TransactionModule.prototype);
    Object.assign(this, ContractModule.prototype);
    Object.assign(this, UtilsModule.prototype);
  }
}
export default SolanaHelperExtended;
