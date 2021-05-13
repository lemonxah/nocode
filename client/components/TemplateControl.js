import Rete from 'rete';
import CustomNode from '@/components/CustomNode.vue';
import VueTemplateControl from '@/components/VueTemplateControl.vue';

class TemplateControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.data.Component = CustomNode;
    this.component = VueTemplateControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default TemplateControl;
