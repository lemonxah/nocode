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
import { anySocket, numSocket } from '@/components/Sockets';

class ToNumComponent extends Rete.Component {
  constructor() {
    super('ToNumber');
  }

  builder(node) {
    const input = new Rete.Input('data', 'Data', anySocket);
    const out = new Rete.Output('num', 'Number', numSocket);
    return node.addInput(input).addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ToNumComponent;
