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
import { numSocket, jsonSocket } from '@/components/Sockets';

class ArrayCountComponent extends Rete.Component {
  constructor() {
    super('Array Count');
  }

  builder(node) {
    // const actin = new Rete.Input('action', 'Action', actionSocket, true);
    // const actout = new Rete.Output('action', 'Action', actionSocket);

    const data = new Rete.Input('payload', 'JSON Payload', jsonSocket);
    const out = new Rete.Output('num', 'Count', numSocket);
    return node
      // .addInput(actin)
      // .addOutput(actout)
      .addInput(data)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ArrayCountComponent;
