import Vue from 'vue';
import axios from 'axios';
import VueAxios from 'vue-axios';
import VueRouter from 'vue-router';
import Index from '@/views/Index.vue';
import EditFlow from '@/views/EditFlow.vue';
import '@/assets/css/tailwind.css';
import App from '@/App.vue';
import store from '@/store';

Vue.use(VueRouter);
Vue.use(VueAxios, axios);

const router = new VueRouter({
  mode: 'history',
  routes: [
    { path: '/flowview', component: Index, name: 'index' },
    { path: '/flowview/edit/:flow_name', component: EditFlow, name: 'edit' },
  ],
});

Vue.config.productionTip = false;
new Vue({
  store,
  router,
  template: '<App/>',
  components: { App },
}).$mount('#app');
