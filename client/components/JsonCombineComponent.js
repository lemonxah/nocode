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

class JsonCombineComponent extends Rete.Component {
  constructor() {
    super('Combine');
  }

  builder(node) {
    const data1 = new Rete.Input('data1', 'Data 1 (data1 if no name)', anySocket);
    const data2 = new Rete.Input('data2', 'Data 2 (data2 if no name)', anySocket);
    const data3 = new Rete.Input('data3', 'Data 3 (data3 if no name)', anySocket);
    const data4 = new Rete.Input('data4', 'Data 4 (data3 if no name)', anySocket);
    const out = new Rete.Output('json', 'JSON Payload', jsonSocket);
    return node
      .addControl(new NameControl(this.editor, 'name1'))
      .addControl(new NameControl(this.editor, 'name2'))
      .addControl(new NameControl(this.editor, 'name3'))
      .addControl(new NameControl(this.editor, 'name4'))
      .addInput(data1)
      .addInput(data2)
      .addInput(data3)
      .addInput(data4)
      .addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default JsonCombineComponent;
