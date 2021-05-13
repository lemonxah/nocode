/* eslint
class-methods-use-this: [
  "error",{
    "exceptMethods": ["worker"]
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
import { numSocket } from '@/components/Sockets';
import NumControl from '@/components/NumControl';

class NumComponent extends Rete.Component {
  constructor() {
    super('Number');
    this.data.Component = CustomNode;
  }

  builder(node) {
    const out = new Rete.Output('num', 'Number', numSocket);
    return node.addControl(new NumControl(this.editor, 'num')).addOutput(out);
  }

  worker(node, inputs, outputs) {
    outputs.num = node.data.num;
  }
}

export default NumComponent;
