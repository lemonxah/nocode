import Rete from 'rete';
import CustomNode from '@/components/CustomNode.vue';
import VueTextControl from '@/components/VueTextControl.vue';

class NameControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.data.Component = CustomNode;
    this.component = VueTextControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default NameControl;
