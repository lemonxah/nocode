<template>
  <div class="flex-col h-full">
    <div class="w-full bg-gray-700">
      <button class="py-2 px-4 m-2 bg-blue-500 border-blue-800 text-white font-medium rounded" @click="onArrange">
        Arrange
      </button>
      <button class="py-2 px-4 m-2 bg-blue-500 border-blue-800 text-white font-medium rounded" @click="onRuleTest">
        Test
      </button>
      <button class="py-2 px-4 m-2 bg-blue-500 border-blue-800 text-white font-medium rounded" @click="onRuleSave">
        Save
      </button>
    </div>
    <div class="flex-col w-full content overflow-hidden">
      <div class="flex">
        <div class="flex-1">
          <vue-json-editor v-model="payload" :show-btns="false" :mode="'code'" />
        </div>
        <div class="flex-1">
          <vue-json-editor v-model="output" :show-btns="false" :mode="'code'" :options="options" />
        </div>
      </div>
      <div id="rete" class="w-full" />
    </div>
  </div>
</template>

<script>
/* eslint
no-unused-vars: ["error", { "args": "none", vars: "all" }]
no-else-return: "error"
*/

import Rete from 'rete';
import ConnectionPlugin from 'rete-connection-plugin';
import VueRenderPlugin from 'rete-vue-render-plugin';
import AreaPlugin from 'rete-area-plugin';
import ContextMenuPlugin from 'rete-context-menu-plugin';
import AutoArrangePlugin from 'rete-auto-arrange-plugin';
import ConnectionMasteryPlugin from 'rete-connection-mastery-plugin';
import MinimapPlugin from 'rete-minimap-plugin';

import vueJsonEditor from 'vue-json-editor';

import NumComponent from '@/components/NumComponent';
import TextComponent from '@/components/TextComponent';
import MongoDBComponent from '@/components/MongoDBComponent';
import ScriptComponent from '@/components/ScriptComponent';
import JsonComponent from '@/components/JsonComponent';
import InputComponent from '@/components/InputComponent';
import OutputComponent from '@/components/OutputComponent';
import TemplateComponent from '@/components/TemplateComponent';
import JsonTemplateComponent from '@/components/JsonTemplateComponent';
import JsonConvertComponent from '@/components/JsonConvertComponent';
import JsonCombineComponent from '@/components/JsonCombineComponent';

export default {
  ssr: false,
  components: {
    vueJsonEditor,
  },
  data() {
    return {
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
    this.editor.use(AutoArrangePlugin, { margin: { x: 50, y: 50 }, depth: 0 });
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
        if (component.name === 'Text' || component.name === 'Number' || component.name === 'JSON') {
          return ['Variables'];
        } else if (component.name === 'Input' || component.name === 'Output') {
          return null;
        } else if (component.name === 'Combine' || component.name === 'JSON' || component.name === 'Convert') {
          return ['JSON'];
        } else if (component.name === 'MongoDB') {
          return ['Database'];
        } else if (component.name === 'Script') {
          return ['Scripting'];
        } else if (component.name === 'JsonTemplate' || component.name === 'Template') {
          return ['Templates'];
        } else if (component.name === 'Contains' || component.name === 'Match') {
          return ['Control'];
        }
        return ['Other'];
      },
    });
    this.editor.trigger('process');
    const components = [
      new InputComponent(),
      new OutputComponent(),
      new NumComponent(),
      new TextComponent(),
      new MongoDBComponent(),
      new ScriptComponent(),
      new JsonComponent(),
      new TemplateComponent(),
      new JsonTemplateComponent(),
      new JsonConvertComponent(),
      new JsonCombineComponent(),
    ];
    components.forEach((c) => this.editor.register(c));
    this.editor.on('process noderemoved nodecreated connectioncreated connectionremoved', () => {
      // console.log(this.editor.toJSON())
    });
    console.log(process.env);
    try {
      const api = `${process.env.VUE_APP_API_URL}/v1/rules/${this.$route.params.rule_name}`;
      const res = await this.axios.get(api, {
        withCredentials: true,
      }).then((response) => {
        console.log(response);
        if (response.data?.rule) {
          this.rule_data = response.data.rule;
          this.payload = response.data.payload;
        }
      });
      console.log(res);
    } catch (e) {
      console.log(e);
    }
    this.editor.fromJSON(this.rule_data);
    this.editor.view.resize();
    this.editor.on('zoom', ({ source }) => source !== 'dblclick');
  },
  methods: {
    onArrange() {
      this.editor.trigger('arrange', { node: this.editor.nodes[0] });
    },
    async onRuleSave() {
      try {
        const api = `${process.env.VUE_APP_API_URL}/v1/rules`;
        const res = await this.axios.post(api, {
          name: this.$route.params.rule_name,
          payload: this.payload,
          rule: this.editor.toJSON(),
        }, {
          withCredentials: true,
        });
        console.log(res);
      } catch (e) {
        console.log(e);
      }
    },
    async onRuleTest() {
      try {
        const api = `${process.env.VUE_APP_API_URL}/v1/ruletest`;
        const res = await this.axios.post(api, {
          payload: this.payload,
          rule: this.editor.toJSON(),
        }, {
          withCredentials: true,
        });
        this.output = res.data;
      } catch (e) {
        console.log(e);
        this.output = {
          error: 'some error occured',
        };
      }
    },
  },
};
</script>
<style>

.content {
  height: 93%;
}

.content .socket.string {
  background: #797979;
  border-color: black;
}
.content .socket.number {
  background: #3773f3;
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
#rete .node.mongodb {
  background: #1eb600;
  border-color: #0d4d00;
}
#rete .node.template {
  background: #315855;
  border-color: #213b39;
}
#rete .node.json-template {
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
#rete .node.convert {
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
