import Rete from 'rete';
import CustomNode from '@/components/CustomNode.vue';
import VueConditionControl from '@/components/VueConditionControl.vue';

class ConditionControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.data.Component = CustomNode;
    this.component = VueConditionControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default ConditionControl;
