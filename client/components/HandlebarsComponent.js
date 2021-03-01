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
import { templateSocket, txtSocket, jsonSocket } from '@/components/Sockets';

class HandlebarsComponent extends Rete.Component {
  constructor() {
    super('Handlebars');
  }

  builder(node) {
    const inp1 = new Rete.Input('payload', 'Json Payload ', jsonSocket);

    const template = new Rete.Input('template', 'Template', templateSocket);
    const out = new Rete.Output('output', 'Text Output', txtSocket);
    const jsonout = new Rete.Output('json', 'JSON Output', jsonSocket);

    return node
      .addInput(inp1)
      .addInput(template)
      .addOutput(out)
      .addOutput(jsonout);
  }

  worker(node, inputs, outputs) { }
}

export default HandlebarsComponent;