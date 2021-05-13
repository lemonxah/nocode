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
import CustomNode from '@/components/CustomNode.vue';
import { numSocket, jsonSocket } from '@/components/Sockets';
import NumControl from '@/components/NumControl';

class ArrayNthComponent extends Rete.Component {
  constructor() {
    super('Nth');
    this.data.Component = CustomNode;
  }

  builder(node) {
    // const actin = new Rete.Input('action', 'Action', actionSocket, true);
    // const actout = new Rete.Output('action', 'Action', actionSocket);
    const data = new Rete.Input('payload', 'JSON Payload', jsonSocket);
    const nth = new Rete.Input('nth', 'Nth element', numSocket);
    nth.addControl(new NumControl(this.editor, 'nth'));
    const out = new Rete.Output('json', 'JSON Payload', jsonSocket);
    return node
      // .addInput(actin)
      // .addOutput(actout)
      .addInput(data)
      .addInput(nth)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ArrayNthComponent;
