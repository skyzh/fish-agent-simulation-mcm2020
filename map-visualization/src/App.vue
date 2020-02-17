<template>
  <div id="app">
    <!-- <img class="float" src="./assets/2049-9.png" /> -->
    <img class="float" src="../../png_process/result/herring/2040-9.png" />
    <img class="float" src="./assets/NASAM_AVG.png" />
    <svg class="float" ref="d3-svg" />
  </div>
</template>

<script lang="ts">
import { Component, Vue, Ref } from "vue-property-decorator";
const renderFish = require('./render');
const renderLand = require('./render_land');
const renderGrid = require('./render_grid');
const renderCircle = require('./render_circle');
const analyzeFish = require('./analyze');

@Component({
  components: {}
})
export default class App extends Vue {
  @Ref("d3-svg") readonly svg!: SVGElement;
  mounted() {
    renderGrid(this.svg)
    renderFish(this.svg)
    // renderLand(this.svg)
    renderCircle(this.svg)
    analyzeFish()
  }
}
</script>

<style lang="scss">
.float {
  position: absolute;
}
body {
  background-color: #ccc
}
</style>
