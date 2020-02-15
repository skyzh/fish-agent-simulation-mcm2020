<template>
  <div id="app">
    <img class="float" src="./assets/2049-9.png" />
    <img class="float" src="./assets/NASAM_AVG.png" />
    <svg class="float" ref="d3-svg" />
  </div>
</template>

<script lang="ts">
import { Component, Vue, Ref } from "vue-property-decorator";
const render = require('./render');

@Component({
  components: {}
})
export default class App extends Vue {
  @Ref("d3-svg") readonly svg!: SVGElement;
  mounted() {
    render(this.svg)
  }
}
</script>

<style lang="scss">
.float {
  position: absolute;
}
</style>
