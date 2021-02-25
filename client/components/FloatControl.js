import Rete from 'rete';
import VueFloatControl from '@/components/VueFloatControl.vue';

class FloatControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.component = VueFloatControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default FloatControl;
