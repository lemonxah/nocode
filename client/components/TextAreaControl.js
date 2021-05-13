import Rete from 'rete';
import CustomNode from '@/components/CustomNode.vue';
import VueTextAreaControl from '@/components/VueTextAreaControl.vue';

class TextAreaControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.data.Component = CustomNode;
    this.component = VueTextAreaControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default TextAreaControl;
