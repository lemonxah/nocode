import Rete from 'rete';
import CustomNode from '@/components/CustomNode.vue';
import VueNumControl from '@/components/VueNumControl.vue';

class NumControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.data.Component = CustomNode;
    this.component = VueNumControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default NumControl;
