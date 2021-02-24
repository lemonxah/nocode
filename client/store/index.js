import Vue from 'vue';
import Vuex from 'vuex';
import jwtdecode from 'vue-jwt-decode';
import axios from 'axios';

Vue.use(Vuex);

const checkToken = async (state, dispatch) => {
  if (!state?.jwt || state?.jwt?.exp <= (new Date().getTime() + 60000)) {
    await dispatch('refresh');
  }
};

export default new Vuex.Store({
  state: {
  },
  mutations: {
    login(state, jwtstring) {
      state.jwt = jwtdecode.decode(jwtstring);
      state.jwtstring = jwtstring;
    },
    logout(state) {
      state.jwt = '';
      state.jwtstring = '';
    },
  },
  actions: {
    async checkRefresh({ state, dispatch }) {
      await checkToken(state, dispatch);
    },
    async refresh({ commit }) {
      try {
        const result = await axios.get(`${process.env.VUE_APP_API_URL}/v1/users/token`, {
          withCredentials: true,
        });
        // eslint-disable-next-line
        const jwtstring = result.data.data[0].access_token;
        commit('login', jwtstring);
      } catch (e) {
        commit('logout');
      }
    },
    async getRule({ state, dispatch }, name) {
      await checkToken(state, dispatch);
      const res = await axios.get(`${process.env.VUE_APP_API_URL}/v1/rules/${name}`, {
        withCredentials: true,
      });
      return res.data;
    },
    async getRules({ state, dispatch }) {
      await checkToken(state, dispatch);
      const res = await axios.get(`${process.env.VUE_APP_API_URL}/v1/rules`, {
        withCredentials: true,
      });
      return res.data;
    },
    async saveRule({ state, dispatch }, { name, payload, rule }) {
      await checkToken(state, dispatch);
      await axios.post(`${process.env.VUE_APP_API_URL}/v1/rules/`, {
        name,
        payload,
        rule,
      }, {
        withCredentials: true,
      });
    },
    async testRule({ state, dispatch }, { payload, rule }) {
      await checkToken(state, dispatch);
      const res = await axios.post(`${process.env.VUE_APP_API_URL}/v1/ruletest`, {
        payload,
        rule,
      }, {
        withCredentials: true,
      });
      return res.data;
    },
  },
  modules: {
  },
});
