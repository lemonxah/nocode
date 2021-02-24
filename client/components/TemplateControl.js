import Rete from 'rete';
import VueTemplateControl from '@/components/VueTemplateControl.vue';

class TemplateControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.component = VueTemplateControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default TemplateControl;
