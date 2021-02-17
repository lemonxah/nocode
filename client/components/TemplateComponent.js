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

class TemplateComponent extends Rete.Component {
  constructor() {
    super('Template');
  }

  builder(node) {
    const inp1 = new Rete.Input('payload', 'Json Payload ', jsonSocket);

    const template = new Rete.Input('template', 'Template', txtSocket);
    template.addControl(new NameControl(this.editor, 'template'));
    const out = new Rete.Output('output', 'Text Output', txtSocket);

    return node
      .addInput(inp1)
      .addInput(template)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default TemplateComponent;
