const path = require('path');

module.exports = {
  env: {
    apiUrl: process.env.API_URL,
  },
  publicRuntimeConfig: {
    apiUrl: process.env.API_URL,
  },
  publicPath: '/v1/ruleview',
  runtimeCompiler: true,
  chainWebpack: (config) => {
    config
      .entry('app')
      .clear()
      .add('./client/main.js')
      .end();
    config.resolve.alias
      .set('@', path.join(__dirname, './client'));
  },
};
