/* eslint-disable no-unused-vars */
import Vue from 'vue';
import Vuex from 'vuex';
import jwtdecode from 'vue-jwt-decode';
import axios from 'axios';

Vue.use(Vuex);

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
    async getFlow(_context, name) {
      const res = await axios.get(`${process.env.VUE_APP_API_URL}/flows/${name}`, {
        withCredentials: true,
      });
      return res.data;
    },
    async getFlowRev(_context, { name, rev }) {
      const res = await axios.get(`${process.env.VUE_APP_API_URL}/flows/${name}?rev=${rev}`, {
        withCredentials: true,
      });
      return res.data;
    },
    async getFlows(_context) {
      const res = await axios.get(`${process.env.VUE_APP_API_URL}/flows`, {
        withCredentials: true,
      });
      return res.data;
    },
    async getFlowMeta(_context, { name }) {
      const res = await axios.get(`${process.env.VUE_APP_API_URL}/flows?name=${name}`, {
        withCredentials: true,
      });
      return res.data?.[0];
    },
    async saveFlow(_context, { name, payload, flow }) {
      const res = await axios.post(`${process.env.VUE_APP_API_URL}/flows/`, {
        name,
        payload,
        flow,
      }, {
        withCredentials: true,
      });
      return res.data;
    },
    async setActive(_context, { name, rev }) {
      await axios.post(`${process.env.VUE_APP_API_URL}/flows/${name}/setactive`, {
        rev,
      }, {
        withCredentials: true,
      });
    },
    async testFlow(_context, { payload, flow }) {
      const res = await axios.post(`${process.env.VUE_APP_API_URL}/flowtest`, {
        payload,
        flow,
      }, {
        withCredentials: true,
      });
      return res.data;
    },
  },
  modules: {
  },
});
