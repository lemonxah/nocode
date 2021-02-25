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

class ArrayFlattenComponent extends Rete.Component {
  constructor() {
    super('Array Flatten');
  }

  builder(node) {
    const data = new Rete.Input('payload', 'JSON Payload', jsonSocket);
    const out = new Rete.Output('json', 'JSON Payload', jsonSocket);
    return node
      .addInput(data)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ArrayFlattenComponent;
