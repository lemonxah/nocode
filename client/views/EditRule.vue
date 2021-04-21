<template>
  <div class="flex-col h-screen overflow-hidden">
    <div class="w-full bg-gray-700 flex">
      <div class="py-2 px-4 ">
        <span class="text-white text-3xl">{{this.$route.params.rule_name}} rule</span>
      </div>
      <div class="flex-1"/>
      <button class="py-2 px-4 m-2 bg-blue-500 border-blue-800 text-white font-medium rounded" @click="onRuleTest">
        Test
      </button>
      <div class="ml-1 mr-1 w-1 h-10 bg-white mt-2 rounded"/>
      <button class="py-2 px-4 m-2 bg-blue-500 border-blue-800 text-white font-medium rounded" @click="onRuleImport">
        Import
        <input type="file" id="file" ref="file" style="display:none" v-on:change="handleFileUpload()"/>
      </button>
      <button class="py-2 px-4 m-2 bg-blue-500 border-blue-800 text-white font-medium rounded" @click="onRuleExport">
        <a id="downloadAnchorElem" style="display:none"></a>
        Export
      </button>
      <div class="ml-1 mr-1 w-1 h-10 bg-white mt-2 rounded"/>
      <button class="py-2 px-4 m-2 bg-blue-500 border-blue-800 text-white font-medium rounded" @click="onRuleSave">
        Save
      </button>
      <div class="ml-1 mr-1 w-1 h-10 bg-white mt-2 rounded"/>
      <button class="py-2 px-4 m-2 bg-blue-500 border-blue-800 text-white font-medium rounded" @click="onJsonToggle">
        {{jsonVisible ? 'Hide Json' : 'Show Json'}}
      </button>
    </div>
    <div class="flex-col flex-nowrap h-full w-full overflow-hidden">
      <div :class="[jsonVisible ? 'flex' : 'hidden']">
        <div class="flex-1 h-full">
          <vue-json-editor v-model="payload" :show-btns="false" :mode="'code'" />
        </div>
        <div class="flex-1">
          <vue-json-editor v-model="output" :show-btns="false" :mode="'code'" :options="options" />
        </div>
      </div>
      <div id="rete" class="w-full h-full overflow-hidden" />
    </div>
  </div>
</template>

<script>
/* eslint
no-unused-vars: 0
no-else-return: "error"
*/

import Rete from 'rete';
import ConnectionPlugin from 'rete-connection-plugin';
import VueRenderPlugin from 'rete-vue-render-plugin';
import AreaPlugin from 'rete-area-plugin';
import ContextMenuPlugin from 'rete-context-menu-plugin';
import ConnectionMasteryPlugin from 'rete-connection-mastery-plugin';
import MinimapPlugin from 'rete-minimap-plugin';

import vueJsonEditor from 'vue-json-editor';

import NumComponent from '@/components/NumComponent';
import TextComponent from '@/components/TextComponent';
import FloatComponent from '@/components/FloatComponent';
import JsonCombineComponent from '@/components/JsonCombineComponent';
import JsonComponent from '@/components/JsonComponent';

import MongoDBGetComponent from '@/components/MongoDBGetComponent';
import MongoDBInsertComponent from '@/components/MongoDBInsertComponent';
import MongoDBUpdateComponent from '@/components/MongoDBUpdateComponent';
import MongoDBReplaceComponent from '@/components/MongoDBReplaceComponent';

import ScriptComponent from '@/components/ScriptComponent';

import InputComponent from '@/components/InputComponent';
import OutputComponent from '@/components/OutputComponent';

import TemplateComponent from '@/components/TemplateComponent';
import HandlebarsComponent from '@/components/HandlebarsComponent';

import ArrayHeadComponent from '@/components/ArrayHeadComponent';
import ArrayNthComponent from '@/components/ArrayNthComponent';
import ArrayMapComponent from '@/components/ArrayMapComponent';
import ArraySumComponent from '@/components/ArraySumComponent';
import ArrayCountComponent from '@/components/ArrayCountComponent';
import ArrayFlattenComponent from '@/components/ArrayFlattenComponent';

import ToJsonComponent from '@/components/ToJsonComponent';
import ToFloatComponent from '@/components/ToFloatComponent';
import ToNumComponent from '@/components/ToNumComponent';
import ToTextComponent from '@/components/ToTextComponent';

import ConditionComponent from '@/components/ConditionComponent';

import { mapActions } from 'vuex';

export default {
  ssr: false,
  components: {
    vueJsonEditor,
  },
  data() {
    return {
      jsonVisible: true,
      file: '',
      payload: {
        somedata: 'hello',
      },
      output: {},
      editor: null,
      options: {
        mode: 'application/json',
        readOnly: true,
      },
      rule_data: {
        id: 'rules@1.0.0',
        nodes: {
          1: {
            id: 1,
            data: {},
            inputs: {},
            outputs: {
              payload: {
                connections: [{
                  node: 2,
                  input: 'payload',
                  data: {},
                }],
              },
            },
            position: [255, 180],
            name: 'Input',
          },
          2: {
            id: 2,
            data: {},
            inputs: {
              payload: {
                connections: [{
                  node: 1,
                  output: 'payload',
                  data: {},
                }],
              },
              status: {
                connections: [{
                  node: 3,
                  output: 'num',
                  data: {},
                }],
              },
            },
            outputs: {},
            position: [600, 180],
            name: 'Output',
          },
          3: {
            id: 3,
            data: {
              num: 200,
            },
            inputs: {},
            outputs: {
              num: {
                connections: [{
                  node: 2,
                  input: 'status',
                  data: {},
                }],
              },
            },
            position: [220, 280],
            name: 'Number',
          },
        },
      },
    };
  },
  async mounted() {
    const container = document.querySelector('#rete');
    this.editor = new Rete.NodeEditor('rules@1.0.0', container);
    this.editor.use(ConnectionPlugin, { curvature: 0.4 });
    this.editor.use(VueRenderPlugin);
    this.editor.use(ConnectionMasteryPlugin);
    this.editor.use(MinimapPlugin);
    this.editor.use(AreaPlugin, {
      background: true,
      scaleExtent: { min: 0.2, max: 2 },
    });
    this.editor.use(ContextMenuPlugin, {
      searchBar: false,
      searchKeep: (title) => true,
      delay: 200,
      nodeItems: (node) => {
        if (node.name === 'Input' || node.name === 'Output') {
          return {
            Delete: false,
            Clone: false,
          };
        }
        return {};
      },
      allocate(component) {
        if (['JSON', 'Number', 'Text', 'Combine', 'Float'].includes(component.name)) {
          return ['Variables'];
        } else if (['Head', 'Nth', 'Array Map', 'Array Sum', 'Array Flatten', 'Array Count'].includes(component.name)) {
          return ['Array'];
        } else if (['Condition'].includes(component.name)) {
          return ['Control'];
        } else if (['ToJSON', 'ToFloat', 'ToText', 'ToNumber'].includes(component.name)) {
          return ['Convert'];
        } else if (['MongoDB Get', 'MongoDB Insert', 'MongoDB Replace', 'MongoDB Update'].includes(component.name)) {
          return ['MongoDB'];
        } else if (['Script'].includes(component.name)) {
          return ['Scripting'];
        } else if (['Handlebars', 'Template'].includes(component.name)) {
          return ['Templates'];
        } else if (['Includes'].includes(component.name)) {
          return ['Control'];
        } else if (['Input', 'Output'].includes(component.name)) {
          return null;
        }
        return ['Other'];
      },
      rename(component) {
        switch (component.name) {
          case '':
            return '';
          case 'Pass Through Action':
            return 'Pass Through';
          case 'Array Sum':
            return 'Sum';
          case 'Array Count':
            return 'Count';
          case 'Array Flatten':
            return 'Flatten';
          case 'Array Map':
            return 'Map';
          case 'MongoDB Get':
            return 'Get';
          case 'MongoDB Insert':
            return 'Insert';
          case 'MongoDB Update':
            return 'Update';
          case 'MongoDB Replace':
            return 'Replace';
          default:
            return component.name;
        }
      },
    });
    this.editor.trigger('process');
    const components = [
      new InputComponent(),
      new OutputComponent(),
      new NumComponent(),
      new TextComponent(),
      new FloatComponent(),

      new ToJsonComponent(),
      new ToFloatComponent(),
      new ToTextComponent(),
      new ToNumComponent(),

      new ConditionComponent(),

      new ArrayHeadComponent(),
      new ArrayNthComponent(),
      new ArrayMapComponent(),
      new ArraySumComponent(),
      new ArrayFlattenComponent(),
      new ArrayCountComponent(),

      new MongoDBGetComponent(),
      new MongoDBInsertComponent(),
      new MongoDBUpdateComponent(),
      new MongoDBReplaceComponent(),

      new ScriptComponent(),
      new JsonComponent(),
      new TemplateComponent(),
      new HandlebarsComponent(),
      new JsonCombineComponent(),
    ];
    components.forEach((c) => this.editor.register(c));
    this.editor.on('process noderemoved nodecreated connectioncreated connectionremoved', () => {
      // console.log(this.editor.toJSON())
    });
    try {
      const res = await this.getRule(this.$route.params.rule_name);
      if (res.rule) {
        this.rule_data = res.rule;
        this.payload = res.payload;
      }
    } catch (e) {
      console.log(e);
    }
    this.editor.fromJSON(this.rule_data);
    // this.editor.view.resize();
    this.editor.on('zoom', ({ source }) => source !== 'dblclick');
  },
  updated() {
    // this.editor.view.resize();
  },
  methods: {
    ...mapActions(['getRule', 'saveRule', 'testRule']),
    handleFileUpload() {
      const [file, ...rest] = this.$refs.file.files;
      const reader = new FileReader();
      reader.readAsText(file, 'UTF-8');
      reader.onload = (evt) => {
        const data = JSON.parse(evt.target.result);
        this.payload = data?.payload;
        this.rule_data = data?.rule;
        this.editor.fromJSON(this.rule_data);
      };
    },
    onRuleImport() {
      const fileupload = document.getElementById('file');
      fileupload.click();
    },
    onRuleExport() {
      const dataStr = `data:text/json;charset=utf-8,${encodeURIComponent(JSON.stringify({ payload: this.payload, rule: this.editor.toJSON() }, null, 2))}`;
      const dlAnchorElem = document.getElementById('downloadAnchorElem');
      dlAnchorElem.setAttribute('href', dataStr);
      dlAnchorElem.setAttribute('download', `${this.$route.params.rule_name}.json`);
      dlAnchorElem.click();
    },
    onJsonToggle() {
      this.jsonVisible = !this.jsonVisible;
    },
    async onRuleSave() {
      try {
        const res = await this.saveRule({
          name: this.$route.params.rule_name,
          payload: this.payload,
          rule: this.editor.toJSON(),
        });
      } catch (e) {
        console.log(e);
      }
    },
    async onRuleTest() {
      try {
        this.output = {
          processing: true,
        };
        const res = await this.testRule({
          payload: this.payload,
          rule: this.editor.toJSON(),
        });
        this.output = res;
      } catch (e) {
        console.log(e);
        this.output = {
          processing: false,
          error: e,
        };
      }
    },
  },
};
</script>
<style>

.content {
  height: 100%;
  overflow: hidden;
}
.content .socket.string {
  background: #797979;
  border-color: black;
}
.content .socket.number {
  background: #3773f3;
  border-color: black;
}
.content .socket.float {
  background: #32bcf3;
  border-color: black;
}
.content .socket.any {
  background: #98ff78;
  border-color: black;
}
.content .socket.json {
  background: #363636;
  border-color: black;
}
.content .socket.template {
  background: #e7e302;
  border-color: black;
}
.content .socket.action {
  background: white;
  border-color: grey;
  border-radius: 3px;
  width: 15px;
}
#rete .node {
  background:#2b9baa;
  border-color: #1a5d66;
}
#rete .node.input {
  background: #e07f00;
  border-color: #9e5a00;
}
#rete .node.output {
  background: #e07f00;
  border-color: #9e5a00;
}
#rete .node.number {
  background: #1765ca;
  border-color: #0f4183;
}
#rete .node.tonumber {
  background: #1765ca;
  border-color: #0f4183;
}
#rete .node.tofloat {
  background: #32bcf3;
  border-color: #207ca0;
}
#rete .node.mongodb-get {
  background: #1eb600;
  border-color: #0d4d00;
}
#rete .node.mongodb-insert {
  background: #1eb600;
  border-color: #0d4d00;
}
#rete .node.mongodb-update {
  background: #1eb600;
  border-color: #0d4d00;
}
#rete .node.mongodb-replace {
  background: #1eb600;
  border-color: #0d4d00;
}
#rete .node.template {
  background: #363636;
  border-color: #2d2d2d;
}
#rete .node.handlebars {
  background: #315855;
  border-color: #213b39;
}
#rete .node.script {
  background: #363636;
  border-color: #2d2d2d;
}
#rete .node.json {
  background: #363636;
  border-color: #2d2d2d;
}
#rete .node.array-map {
  background: #363636;
  border-color: #2d2d2d;
}
#rete .node.array-flatten {
  background: #363636;
  border-color: #2d2d2d;
}
#rete .node.array-sum {
  background: #363636;
  border-color: #2d2d2d;
}
#rete .node.array-count {
  background: #363636;
  border-color: #2d2d2d;
}
#rete .node.tojson {
  background: #363636;
  border-color: #2d2d2d;
}
#rete .node.combine {
  background: #363636;
  border-color: #2d2d2d;
}
#rete .node.text {
  background: #797979;
  border-color: #363636;
}
.content .input-control input {
  width: 140px;
}
</style>
