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

class InputComponent extends Rete.Component {
  constructor() {
    super('Input');
  }

  builder(node) {
    const out = new Rete.Output('payload', 'REST Payload', jsonSocket);

    return node
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default InputComponent;
