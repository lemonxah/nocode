const path = require('path');

module.exports = {
  publicPath: '/flowview',
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
