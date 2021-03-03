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
import {
  floatSocket,
  txtSocket,
  jsonSocket,
  // actionSocket,
} from '@/components/Sockets';

class ArraySumComponent extends Rete.Component {
  constructor() {
    super('Array Sum');
  }

  builder(node) {
    // const actin = new Rete.Input('action', 'Action', actionSocket, true);
    // const actout = new Rete.Output('action', 'Action', actionSocket);
    const data = new Rete.Input('payload', 'JSON Payload', jsonSocket);
    const field = new Rete.Input('field', 'Field Name(Empty if no field)', txtSocket);
    const out = new Rete.Output('float', 'Sum Float', floatSocket);
    return node
      // .addInput(actin)
      // .addOutput(actout)
      .addInput(data)
      .addInput(field)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ArraySumComponent;
