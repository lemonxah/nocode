<template>
  <div id="app" class="mscreen flow-root">
    <div class="w-full h-auto bg-gray-700">
      <button class="py-2 px-4 m-2 bg-blue-500 border-blue-800 text-white font-medium rounded" @click="onRuleTest">
        Test
      </button>
    </div>
    <div class="flex-col h-full w-full">
      <div class="flex">
        <div class="flex-1">
          <vue-json-editor v-model="payload" :show-btns="false" :mode="'code'" />
        </div>
        <div class="flex-1">
          <vue-json-editor v-model="output" :show-btns="false" :mode="'code'" :options="options" />
        </div>
      </div>
      <div id="rete" class="content h-full w-full flex-1" />
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
    };
  },
  mounted() {
    const container = document.querySelector('#rete');
    this.editor = new Rete.NodeEditor('rules@1.0.0', container);
    this.editor.use(ContextMenuPlugin, {
      searchBar: false, // true by default
      searchKeep: (title) => true, // leave item when searching, optional.
      // For example, title => ['Refresh'].includes(title);
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
    this.editor.use(ConnectionPlugin, { curvature: 0.4 });
    this.editor.use(VueRenderPlugin);
    this.editor.use(AreaPlugin, {
      background: true,
      scaleExtent: { min: 0.2, max: 2 },
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
    // new data
    const jsdata = {
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
          position: [400.9375, 212],
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
          position: [1132.6041720723576, 210.2222269726779],
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
          position: [887.7152676871502, 300.7778008420427],
          name: 'Number',
        },
      },
    };
    this.editor.fromJSON(jsdata);
  },
  methods: {
    async onRuleTest() {
      try {
        // const rulejson = this.editor.toJSON();
        // const body = {
        //   payload: this.payload,
        //   rule: rulejson,
        // };
        // this.output = await this.ruleTest(body);
      } catch (e) {
        this.output = {
          error: 'some error occured',
        };
      }
    },
  },
};
</script>
<style>
.mscreen {
  height: 70vh;
}

.content .socket.string {
  background: #ffee8e;
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
  background: #ff3c00;
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
#rete .node.mongodb {
  background: #2fbdb1;
  border-color: #1b6d66;
}
.content .input-control input {
  width: 140px;
}

</style>
