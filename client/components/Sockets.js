import Rete from 'rete';

const numSocket = new Rete.Socket('Number');
const txtSocket = new Rete.Socket('String');
const scriptSocket = new Rete.Socket('Script');
const boolSocket = new Rete.Socket('Bool');
const actionSocket = new Rete.Socket('Action');
const jsonSocket = new Rete.Socket('Json');
const templateSocket = new Rete.Socket('Template');
const floatSocket = new Rete.Socket('Float');

const anySocket = new Rete.Socket('ANY');
numSocket.combineWith(anySocket);
txtSocket.combineWith(anySocket);
boolSocket.combineWith(anySocket);
jsonSocket.combineWith(anySocket);
floatSocket.combineWith(anySocket);

export {
  numSocket,
  txtSocket,
  scriptSocket,
  boolSocket,
  actionSocket,
  jsonSocket,
  anySocket,
  floatSocket,
  templateSocket,
};
