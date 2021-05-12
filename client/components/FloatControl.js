import Rete from 'rete';
import CustomNode from '@/components/CustomNode.vue';
import VueFloatControl from '@/components/VueFloatControl.vue';

class FloatControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.data.Component = CustomNode;
    this.component = VueFloatControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default FloatControl;
