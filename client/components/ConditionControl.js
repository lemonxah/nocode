import Rete from 'rete';
import VueConditionControl from '@/components/VueConditionControl.vue';

class ConditionControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.component = VueConditionControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default ConditionControl;
