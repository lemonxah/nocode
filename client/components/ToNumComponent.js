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
import { anySocket, numSocket } from '@/components/Sockets';

class ToNumComponent extends Rete.Component {
  constructor() {
    super('ToNumber');
    this.data.Component = CustomNode;
  }

  builder(node) {
    // const actin = new Rete.Input('action', 'Action', actionSocket, true);
    // const actout = new Rete.Output('action', 'Action', actionSocket);
    const input = new Rete.Input('data', 'Data', anySocket);
    const out = new Rete.Output('num', 'Number', numSocket);
    return node
      // .addInput(actin)
      // .addOutput(actout)
      .addInput(input)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ToNumComponent;
