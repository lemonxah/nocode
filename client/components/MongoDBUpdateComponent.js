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
import { txtSocket, actionSocket, jsonSocket } from '@/components/Sockets';

class MongoDBUpdateComponent extends Rete.Component {
  constructor() {
    super('MongoDB Update');
    this.data.limit = 20;
    this.data.query = 'deleted == false';
    this.data.dbname = 'rules';
    this.data.colname = 'cache';
  }

  builder(node) {
    const actin = new Rete.Input('action', 'Action', actionSocket, true);
    // const actout = new Rete.Output('action', 'Action', actionSocket);
    const inp2 = new Rete.Input('colname', 'Collection Name', txtSocket);
    const inp3 = new Rete.Input('query', 'Query String', txtSocket);
    const inp4 = new Rete.Input('payload', 'JSON Payload', jsonSocket);
    const out = new Rete.Output('json', 'Query Result', jsonSocket);

    return node
      .addInput(actin)
      // .addOutput(actout)
      .addInput(inp2)
      .addInput(inp3)
      .addInput(inp4)
      .addOutput(out);
  }

  worker(node, inputs, outputs) {

  }
}

export default MongoDBUpdateComponent;
