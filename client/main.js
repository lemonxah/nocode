import Vue from 'vue';
import axios from 'axios';
import VueAxios from 'vue-axios';
import VueRouter from 'vue-router';
import Index from '@/views/Index.vue';
import EditRule from '@/views/EditRule.vue';
// import store from './store';
import '@/assets/css/tailwind.css';
import App from '@/App.vue';

Vue.use(VueRouter);
Vue.use(VueAxios, axios);

const router = new VueRouter({
  mode: 'history',
  routes: [
    { path: '/v1/ruleview', component: Index },
    { path: '/v1/ruleview/edit/:rule_name', component: EditRule },
  ],
});

Vue.config.productionTip = false;
new Vue({
  // store,
  router,
  template: '<App/>',
  components: { App },
}).$mount('#app');
