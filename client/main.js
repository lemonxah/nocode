import Vue from 'vue';
import axios from 'axios';
import VueAxios from 'vue-axios';
import VueRouter from 'vue-router';
import Index from '@/views/Index.vue';
import EditRule from '@/views/EditRule.vue';
import '@/assets/css/tailwind.css';
import App from '@/App.vue';
import store from '@/store';

Vue.use(VueRouter);
Vue.use(VueAxios, axios);

const router = new VueRouter({
  mode: 'history',
  routes: [
    { path: '/v1/ruleview', component: Index, name: 'index' },
    { path: '/v1/ruleview/edit/:rule_name', component: EditRule, name: 'edit' },
  ],
});

Vue.config.productionTip = false;
new Vue({
  store,
  router,
  template: '<App/>',
  components: { App },
}).$mount('#app');
