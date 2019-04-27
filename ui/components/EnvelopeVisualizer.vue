<template>
    <div ref="visualizer" class="envelope__visualizer">
        <svg class="canvas" :viewBox="'0 0 300 100'" preserveAspectRatio='xMinYMid slice'>
        <path class="envelope-shape" :d="'M0,100L0,0'"></path>
        <circle class="delay" cx=0 cy=100 r=6></circle>
        <circle class="attack" cx=0 cy=0 r=6></circle>
        <circle class="hold" cx=0 cy=0 r=6></circle>
        <circle class="decay" cx=0 cy=100 r=6></circle>
        <circle class="release" cx=0 cy=100 r=6></circle>
        </svg>
    </div>
</template>

<script>
function visualize(root, at, de, su, re) {
    let maxPtSeparation = 75;

    let ho = 1000 // hold is 0.5;
    let delayvalue = 0;

    let sustainvalue = su * 100;

    let scale = 2000;

    let shape = root.querySelector('.envelope-shape');

    let delay = root.querySelector('.delay');
    let attack = root.querySelector('.attack');
    let hold = root.querySelector('.hold');
    let decay = root.querySelector('.decay');
    let sustain = root.querySelector('.sustain');
    let release = root.querySelector('.release');

    let ptDelay = maxPtSeparation * delayvalue / scale;
    let ptAttack = ptDelay + (maxPtSeparation * at / scale);
    let ptHold = ptAttack + (maxPtSeparation * ho / scale);
    let ptDecay = ptHold + (maxPtSeparation * de / scale) * (scale - sustainvalue) / scale;
    let ptSustain = 100 - sustainvalue;
    let ptRelease = ptDecay + (maxPtSeparation * re / scale);

    shape.setAttribute('d',
    `M${ptDelay},100`+
    `C${ptDelay},100,${ptAttack},0,${ptAttack},0`+
    `L${ptHold},0`+
    `C${ptHold},0,${ptDecay},${ptSustain},${ptDecay},${ptSustain}`+
    `C${ptDecay},${ptSustain},${ptRelease},100,${ptRelease},100`);

    delay.setAttribute('cx', ptDelay);
    attack.setAttribute('cx', ptAttack);
    hold.setAttribute('cx', ptHold);
    decay.setAttribute('cx', ptDecay);
    decay.setAttribute('cy', ptSustain);
    release.setAttribute('cx', ptRelease);
}

export default {
  name: 'EnvelopeVisualizer',
  props: {
      max: { type: Number, required: true },
      attack: { type: Number, required: true },
      decay: { type: Number, required: true },
      sustain: { type: Number, required: true },
      release: { type: Number, required: true },
  },
  watch: {
      attack: function() { this.$forceUpdate(); },
      decay: function() { this.$forceUpdate(); },
      sustain: function() { this.$forceUpdate(); },
      release: function() { this.$forceUpdate(); },
  },
  mounted() {
      visualize(this.$refs.visualizer, this.attack, this.decay, this.sustain, this.release);
  },
  updated() {
      visualize(this.$refs.visualizer, this.attack, this.decay, this.sustain, this.release);
  }
}
</script>

<style lang="scss" scoped>
.envelope__visualizer {
  height: 200px;
  background: #21272b;
  border: 2px solid #191f23;
  .canvas {
    width: 100%;
    height: 100%;
    overflow: visible;
    .envelope-shape { fill: transparent; }
    circle { fill: #284554; }
    path, circle {
      stroke: #4eccff;
      stroke-width: 1.0;
    }
  }
}
</style>
