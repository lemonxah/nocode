<template>
  <div class="flow-root h-screen bg-gray-300">
    <div class="w-full bg-gray-700 text-white text-3xl p-5">
      Flows
    </div>
    <div class="ml-1 rounded-md mt-3 bg-gray-700 w-1/3 flex">
      <input class="focus:outline-none flex-1 text-xl p-1" placeholder="new flow name (minimum 4 chars)" v-model="newflow" type="text">
      <div class="rounded bg-gray-700 text-white text-lg p-1">
        <router-link v-if="hasFlowName" :to="{ name: 'edit', params: { flow_name: this.newflow }}" class="">
          New Flow
        </router-link>
        <span v-else class="cursor-pointer" @mousedown.prevent>
          New Flow
        </span>
      </div>
    </div>
    <div class="mt-3 w-1/2">
      <div v-for="item in flows" :key="item.name" class="p-2 bg-gray-700 rounded-md m-1 text-2xl text-white">
        <div class="flex" >
          <div class="my-auto">
            {{ item.name }}
          </div>
          <div class="flex-1"></div>
          <div class="my-auto">
            Revisions
          </div>
          <div class="ml-3 mr-1 w-px h-7 bg-white mt-2 rounded my-auto"/>
          <div class="ml-3 my-auto">
            Active:
          </div>
          <div class="ml-4 my-auto">
            <select name="revisions" id="revisions" class="bg-gray-600 text-white my-auto w-16 focus:outline-none" @change="onActiveChange(item.name, $event)">
              <option v-for="rev in revRange(item.latest_rev)" :key="rev" :selected="rev === item.active_rev" :value="rev" class="text-right">
                {{rev}}
              </option>
            </select>
          </div>
          <div class="ml-4 my-auto">
            Latest:
          </div>
          <div class="ml-4 my-auto w-16 text-right border border-white rounded px-2 py-1">
            {{ item.latest_rev }}
          </div>
          <router-link :to="{ name: 'edit', params: { flow_name: item.name }}" class="rounded cursor-pointer text-white bg-blue-800 py-1 px-4 ml-5">
            Edit
          </router-link>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapActions } from 'vuex';

export default {
  data() {
    return {
      flows: [],
      newflow: '',
    };
  },
  async mounted() {
    try {
      this.flows = await this.getFlows();
    } catch (e) {
      // console.log(e);
    }
  },
  computed: {
    hasFlowName() {
      return this.newflow.length > 3;
    },
  },
  methods: {
    ...mapActions(['getFlows', 'setActive']),
    revRange(max) {
      return [...Array(max).keys()].map((i) => i + 1);
    },
    async onActiveChange(name, event) {
      try {
        const data = {
          name,
          rev: parseInt(event.target.value, 10),
        };
        // console.log(name, event.target.value, data);
        await this.setActive(data);
      } catch (e) {
        // console.log(e)
      }
    },

  },
};
</script>

<style scoped>
  select { text-align-last: right; }
  option { direction: rtl; }
</style>
