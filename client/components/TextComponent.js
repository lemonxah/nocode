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
import { txtSocket } from '@/components/Sockets';
import TextControl from '@/components/TextControl';

class TextComponent extends Rete.Component {
  constructor() {
    super('Text');
    this.data.Component = CustomNode;
  }

  builder(node) {
    const out = new Rete.Output('txt', 'Text', txtSocket);
    return node.addControl(new TextControl(this.editor, 'txt')).addOutput(out);
  }

  worker(node, inputs, outputs) {
    outputs.num = node.data.num;
  }
}

export default TextComponent;
