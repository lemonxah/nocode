/* eslint
no-unused-vars: ["error", { "args": "none" }]
class-methods-use-this: [
  "error",{
    "exceptMethods": ["worker", "builder"]
  }
]
no-param-reassign: [
  "error", {
    "props": true,
    "ignorePropertyModificationsFor": ["outputs"]
  }
]
*/
/* eslint-env es6 */

import Rete from 'rete';
import { jsonSocket } from '@/components/Sockets';

class ArrayHeadComponent extends Rete.Component {
  constructor() {
    super('Head');
  }

  builder(node) {
    // const actin = new Rete.Input('action', 'Action', actionSocket, true);
    // const actout = new Rete.Output('action', 'Action', actionSocket);
    const data = new Rete.Input('payload', 'JSON Array', jsonSocket);
    const out = new Rete.Output('json', 'JSON Payload', jsonSocket);
    return node
      // .addInput(actin)
      // .addOutput(actout)
      .addInput(data)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ArrayHeadComponent;
