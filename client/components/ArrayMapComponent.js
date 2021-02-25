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
import { txtSocket, jsonSocket } from '@/components/Sockets';
import NameControl from '@/components/NameControl';

class ArrayMapComponent extends Rete.Component {
  constructor() {
    super('Array Map');
  }

  builder(node) {
    const data = new Rete.Input('payload', 'JSON Payload', jsonSocket);
    const ret = new Rete.Input('field', 'Field Name', txtSocket);
    ret.addControl(new NameControl(this.editor, 'field'));
    const out = new Rete.Output('json', 'JSON Payload', jsonSocket);
    return node
      .addInput(data)
      .addInput(ret)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ArrayMapComponent;
