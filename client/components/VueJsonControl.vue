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
import 'prismjs/components/prism-clike';
import 'prismjs/components/prism-json';
import 'prismjs/themes/prism-tomorrow.css'; // import syntax highlighting styles

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
      json: {},
      value: '',
      lineNumbers: true,
    };
  },
  watch: {
    value: function (val, oldval) {
      try {
        const json = JSON.parse(val);
        this.json = json;
        this.update();
      } catch (e) {
        // not empty
      }
    },
  },
  mounted() {
    this.value = JSON.stringify(this.getData(this.ikey), null, 2);
  },
  methods: {
    highlighter(code) {
      return highlight(code, languages.json);
    },
    update() {
      if (this.ikey) {
        this.putData(this.ikey, this.json);
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
