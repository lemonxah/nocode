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
import { jsonSocket } from '@/components/Sockets';
import JsonControl from '@/components/JsonControl';

class JsonComponent extends Rete.Component {
  constructor() {
    super('JSON');
    this.data.Component = CustomNode;
  }

  builder(node) {
    const out = new Rete.Output('json', 'JSON Payload', jsonSocket);
    return node
      .addControl(new JsonControl(this.editor, 'json')).addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default JsonComponent;
