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
import NumControl from '@/components/NumControl';

class OutputComponent extends Rete.Component {
  constructor() {
    super('Output');
    this.data.status = 200;
  }

  builder(node) {
    // const actin = new Rete.Input('action', 'Action', actionSocket, true);

    const inp1 = new Rete.Input('payload', 'REST Payload', jsonSocket);
    const inp2 = new Rete.Input('status', 'Status Code', numSocket);

    inp2.addControl(new NumControl(this.editor, 'status'));

    return node
      // .addInput(actin)
      .addInput(inp1)
      .addInput(inp2);
  }

  worker(node, inputs, outputs) { }
}

export default OutputComponent;
