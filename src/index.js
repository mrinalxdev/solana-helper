import SolanaHelper from "./initialization";
import { prototype } from "./account";
import { prototype as _prototype } from "./transaction";
import { prototype as __prototype } from "./contract";

class SolanaHelperExtended extends SolanaHelper {
  constructor(endpoint) {
    super(endpoint);
    Object.assign(this, prototype);
    Object.assign(this, _prototype);
    Object.assign(this, __prototype);
  }
}
export default SolanaHelperExtended;
