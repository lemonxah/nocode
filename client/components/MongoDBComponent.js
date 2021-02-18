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
import { txtSocket, numSocket, jsonSocket } from '@/components/Sockets';

class MongoDBComponent extends Rete.Component {
  constructor() {
    super('MongoDB');
  }

  builder(node) {
    const inp1 = new Rete.Input('dbname', 'Database Name', txtSocket);
    const inp2 = new Rete.Input('colname', 'Collection Name', txtSocket);
    const inp3 = new Rete.Input('query', 'Query String', txtSocket);
    const inp4 = new Rete.Input('limit', 'Limit', numSocket);
    const out = new Rete.Output('json', 'Query Result', jsonSocket);

    return node
      .addInput(inp1)
      .addInput(inp2)
      .addInput(inp3)
      .addInput(inp4)
      .addOutput(out);
  }

  worker(node, inputs, outputs) {

  }
}

export default MongoDBComponent;
