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
import { anySocket, jsonSocket } from '@/components/Sockets';
import NameControl from '@/components/NameControl';

class ToJsonComponent extends Rete.Component {
  constructor() {
    super('ToJSON');
  }

  builder(node) {
    const data = new Rete.Input('data', 'Data', anySocket);
    const out = new Rete.Output('json', 'JSON Payload', jsonSocket);
    return node
      .addControl(new NameControl(this.editor, 'name'))
      .addInput(data)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ToJsonComponent;
