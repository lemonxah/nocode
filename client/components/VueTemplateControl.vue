<template>
  <div @pointerdown.stop @pointermove.stop @wheel.stop @contextmenu.stop class="cursor-text">
    <prism-editor
      class="my-editor height-300"
      v-model="value"
      :highlight="highlighter"
      :line-numbers="lineNumbers"
    ></prism-editor>
  </div>
</template>
<script>
/* eslint-env es6 */
/* eslint
object-shorthand: ["off"]
no-unused-vars: ["error", { "args": "none", vars: "all" }]
no-else-return: "error"
*/
import { PrismEditor } from 'vue-prism-editor';
import 'vue-prism-editor/dist/prismeditor.min.css'; // import the styles somewhere

// import highlighting library (you can use any library you want just return html string)
import { highlight, languages } from 'prismjs/components/prism-core';
import 'prismjs/components/prism-markup';
import 'prismjs/components/prism-markup-templating';
import 'prismjs/components/prism-handlebars';

export default {
  props: {
    readonly: {
      type: Boolean,
      default: () => false,
    },
    emitter: {
      type: Object,
      default: () => {},
    },
    ikey: {
      type: String,
      default: () => '',
    },
    getData: {
      type: Function,
      default: () => {},
    },
    putData: {
      type: Function,
      default: () => {},
    },
  },
  components: {
    PrismEditor,
  },
  data() {
    return {
      value: '',
      lineNumbers: true,
    };
  },
  watch: {
    value: function (val, oldval) {
      try {
        // const json = JSON.parse(val);
        // this.json = json;
        this.update();
      } catch (e) {
        // not empty
      }
    },
  },
  mounted() {
    this.value = this.getData(this.ikey);
  },
  methods: {
    highlighter(code) {
      return highlight(code, languages.handlebars);
    },
    update() {
      if (this.ikey) {
        this.putData(this.ikey, this.value);
      }
      this.emitter.trigger('process');
    },
  },
};
</script>
<style lang="scss">
.my-editor {
  background: #2d2d2d;
  color: #ccc;

  font-family: Fira code, Fira Mono, Consolas, Menlo, Courier, monospace;
  font-size: 14px;
  line-height: 1.5;
  padding: 5px;
  width: 500px;
}

.prism-editor__textarea:focus {
  outline: none;
}

.height-300 {
  height: 400px;
}
</style>
