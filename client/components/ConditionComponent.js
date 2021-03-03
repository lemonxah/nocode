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
import { anySocket, actionSocket } from '@/components/Sockets';
import ConditionControl from '@/components/ConditionControl';

class ConditionComponent extends Rete.Component {
  constructor() {
    super('Condition');
  }

  builder(node) {
    const actin = new Rete.Input('action', 'Action', actionSocket, true);
    const inp1 = new Rete.Input('left', 'left', anySocket);
    const inp2 = new Rete.Input('right', 'right', anySocket);
    const otrue = new Rete.Output('true', 'True', actionSocket);
    const ofalse = new Rete.Output('false', 'False', actionSocket);

    return node
      .addInput(actin)
      .addControl(new ConditionControl(this.editor, 'condition'))
      .addInput(inp1)
      .addInput(inp2)
      .addOutput(otrue)
      .addOutput(ofalse);
  }

  worker(node, inputs, outputs) { }
}

export default ConditionComponent;
