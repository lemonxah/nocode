<template>
  <div class="flow-root h-full bg-gray-300">
    <div class="w-full bg-gray-700 text-white text-3xl p-5">
      Rules
    </div>
    <div class="ml-1 rounded-md mt-3 bg-gray-700 w-1/3 flex">
      <input class="focus:outline-none flex-1 text-xl p-1" placeholder="new rule name (minimum 4 chars)" v-model="newrule" type="text">
      <div class="rounded bg-gray-700 text-white text-lg p-1">
        <router-link v-if="hasRuleName" :to="{ name: 'edit', params: { rule_name: this.newrule }}" class="">
          New Rule
        </router-link>
        <span v-else class="cursor-pointer" @mousedown.prevent>
          New Rule
        </span>
      </div>
    </div>
    <div class="mt-3 w-1/3">
      <div v-for="item in rules" :key="item.name" class="p-2 bg-gray-700 rounded-md m-1 text-2xl text-white cursor-pointer">
        <router-link :to="{ name: 'edit', params: { rule_name: item.name }}">
          <div class="flex" >
            <div>
              {{ item.name }}
            </div>
            <div class="flex-1"></div>
            <div>
              version:
            </div>
            <div class="ml-4">
              1.0
            </div>
          </div>
        </router-link>
      </div>
    </div>
  </div>
</template>

<script>
import { mapActions } from 'vuex';

export default {
  data() {
    return {
      rules: [],
      newrule: '',
    };
  },
  async mounted() {
    try {
      this.rules = await this.getRules();
    } catch (e) {
      console.log(e);
    }
  },
  computed: {
    hasRuleName() {
      return this.newrule.length > 3;
    },
  },
  methods: {
    ...mapActions(['getRules']),
  },
};
</script>
