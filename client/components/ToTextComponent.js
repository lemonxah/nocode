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
import { anySocket, txtSocket } from '@/components/Sockets';

class ToTextComponent extends Rete.Component {
  constructor() {
    super('ToText');
  }

  builder(node) {
    const input = new Rete.Input('data', 'Data', anySocket);
    const out = new Rete.Output('txt', 'Text', txtSocket);
    return node.addInput(input).addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ToTextComponent;
