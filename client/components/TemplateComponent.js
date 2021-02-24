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
import { templateSocket } from '@/components/Sockets';
// import JsonControl from '@/components/JsonControl';
import TemplateControl from '@/components/TemplateControl';

class TemplateComponent extends Rete.Component {
  constructor() {
    super('Template');
  }

  builder(node) {
    const out = new Rete.Output('template', 'Template', templateSocket);
    return node
      .addControl(new TemplateControl(this.editor, 'template')).addOutput(out);
  }

  worker(node, inputs, outputs) { }
}

export default TemplateComponent;
