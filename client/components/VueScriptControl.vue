<template>
  <textarea class="jinp" :readonly="readonly" :value="value" @input="change($event)" @pointerdown.stop @pointermove.stop />
</template>
<script>
export default {
  props: {
    readonly: {
      type: Boolean,
      default: () => false,
    },
    emitter: {
      type: Object,
      default: () => {},
    },
    ikey: {
      type: String,
      default: () => '',
    },
    getData: {
      type: Function,
      default: () => {},
    },
    putData: {
      type: Function,
      default: () => {},
    },
  },
  data() {
    return {
      value: 0,
    };
  },
  mounted() {
    this.value = this.getData(this.ikey);
  },
  methods: {
    change(e) {
      this.value = e.target.value;
      this.update();
    },
    update() {
      if (this.ikey) {
        this.putData(this.ikey, this.value);
      }
      this.emitter.trigger('process');
    },
  },
};
</script>
<style>
select, .jinp {
  border-radius: 10px;
  background-color: white;
  padding: 2px 6px;
  border: 1px solid #999;
  font-size: 110%;
  width: 100%;
}
.node .control textarea, .node .input-control .jinp {
  width: 300px;
  height: 400px;
}

</style>
