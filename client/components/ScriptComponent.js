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
import ScriptControl from '@/components/ScriptControl';

class ScriptComponent extends Rete.Component {
  constructor() {
    super('Script');
  }

  builder(node) {
    const inp1 = new Rete.Input('payload', 'Input', jsonSocket);
    const out = new Rete.Output('payload', 'Output', jsonSocket);

    return node
      .addControl(new ScriptControl(this.editor, 'src'))
      .addInput(inp1)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default ScriptComponent;
