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
import { anySocket, floatSocket } from '@/components/Sockets';

class ToFloatComponent extends Rete.Component {
  constructor() {
    super('ToFloat');
  }

  builder(node) {
    const input = new Rete.Input('data', 'Data', anySocket);
    const out = new Rete.Output('float', 'Float', floatSocket);
    return node.addInput(input).addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ToFloatComponent;
