<template>
  <div class="envelope">
      <EnvelopeVisualizer v-if="visualizer"
        v-bind:max=10
        v-bind:attack="attack"
        v-bind:decay="decay"
        v-bind:sustain="sustain"
        v-bind:release="release"
      />
      <div class="envelope__controls">
          <Knob v-on:change="onAttack" :initial="attack" v-bind:min=0 v-bind:max=2000 label="Attack" ringType='positive'/>
          <Knob v-on:change="onDecay" :initial="decay" v-bind:min=0 v-bind:max=2000 label="Decay" ringType='positive'/>
          <Knob v-on:change="onSustain" :initial="sustain" v-bind:min=0 v-bind:max=1 label="Sustain" ringType='positive'/>
          <Knob v-on:change="onRelease" :initial="release" v-bind:min=0 v-bind:max=2000 label="Release" ringType='positive'/>
      </div>
  </div>
</template>

<script>
import Knob from './Knob.vue';
import Vue from 'vue';
import EnvelopeVisualizer from './EnvelopeVisualizer.vue';
export default {
  name: 'Envelope',
  props: ['adsr', 'visualizer'],
  components: { Knob, EnvelopeVisualizer },
  data: function() {
    console.log(this.adsr);
    return {
      attack: this.adsr[0],
      decay: this.adsr[1],
      sustain: this.adsr[2],
      release: this.adsr[3],
    }
  },
  methods: {
      onAttack: function(v) {
          this.attack = v;
          this.$emit('onAttack', v);
      },
      onDecay: function(v) {
          this.decay = v;
          this.$emit('onDecay', v);
      },
      onSustain: function(v) {
          this.sustain = v;
          this.$emit('onSustain', v);
      },
      onRelease: function(v) {
          this.release = v;
          this.$emit('onRelease', v);
      }
  },
}
</script>

<style lang="scss" scoped>
// FL studio knobs - demo styles
$envelope-knob-size: 80px;
$tension-knob-size: 60px;
.envelope {
  width: 600px;
  padding: 20px;
  overflow: hidden;
  background: #363c40;
  border: 2px solid #272d31;
  box-shadow: 0 10px 70px rgba(#000000, 0.6);
}
.envelope__controls, .envelope__tension {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.envelope__controls {
  padding: 30px 0 0;
}
</style>
