import Rete from 'rete';
import CustomNode from '@/components/CustomNode.vue';
import VueJsonControl from '@/components/VueJsonControl.vue';

class JsonControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.data.Component = CustomNode;
    this.component = VueJsonControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default JsonControl;
