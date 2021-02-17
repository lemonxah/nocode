import Rete from 'rete';
import VueScriptControl from '@/components/VueScriptControl.vue';

class ScriptControl extends Rete.Control {
  constructor(emitter, key, readonly) {
    super(key);
    this.component = VueScriptControl;
    this.props = { emitter, ikey: key, readonly };
  }

  setValue(val) {
    this.vueContext.value = val;
  }
}

export default ScriptControl;
