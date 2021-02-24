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

class ArrayHeadComponent extends Rete.Component {
  constructor() {
    super('Nth');
  }

  builder(node) {
    const data = new Rete.Input('payload', 'JSON Payload', jsonSocket);
    const nth = new Rete.Input('nth', 'Nth element', numSocket);
    nth.addControl(new NumControl(this.editor, 'nth'));
    const out = new Rete.Output('json', 'JSON Payload', jsonSocket);
    return node
      .addInput(data)
      .addInput(nth)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ArrayHeadComponent;
