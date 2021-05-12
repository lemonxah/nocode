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
import { floatSocket } from '@/components/Sockets';
import FloatControl from '@/components/FloatControl';

class NumComponent extends Rete.Component {
  constructor() {
    super('Float');
    this.data.Component = CustomNode;
  }

  builder(node) {
    const out = new Rete.Output('float', 'Float', floatSocket);
    return node.addControl(new FloatControl(this.editor, 'float')).addOutput(out);
  }

  worker(node, inputs, outputs) {
    outputs.num = node.data.num;
  }
}

export default NumComponent;
