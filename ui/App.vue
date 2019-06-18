<template>
  <div id="app">
    <Header v-on:power="onPower"/>
    <Knob v-on:change="onMasterGain" :initial="master_gain" v-bind:min=0 v-bind:max=1.0 label="Master" ringType='positive'/>
    <hr/>
    <Subjam :subjam="subjam" :rust="rust"/>
  </div>
</template>

<script>
import Header from './components/Header.vue'
import Subjam from './components/Subjam.vue'
import Knob from './components/Knob.vue'
export default {
  name: 'App',
  props: ['rust'],
  components: {
    Header,
    Subjam,
    Knob
  },
  data: function() {
    return { subjam: null, mixer: null, master_gain: 0.9 }
  },
  methods: {
    onMasterGain: function(v) {
      this.master_gain = v;
      if (this.mixer) {
        this.mixer.set_master_gain(v);
      }
    },
    noteOn: function(note, velocity) {
      if (this.subjam) {
        this.subjam.note_on(note, velocity);
      }
    },
    noteOff: function(note) {
      if (this.subjam) {
        this.subjam.note_off(note);
      }
    },
    onPower: function(is_on) {
      if (is_on) {
        this.audioContext = new AudioContext();
        this.mixer = new this.rust.Mixer(this.audioContext, 4);
        this.subjam = new this.rust.Subjam(this.audioContext, document.bus);

        console.log(document.trigger);

        this.subjam.connect_to_mixer(this.mixer, 0);
        this.mixer.connect_to_speakers();
        this.$forceUpdate();
      } else {
        this.audioContext.close();
        this.subjam.free();
        this.mixer.free();
        this.subjam = null;
        this.mixer = null;
        this.audioContext = null;
      }
    }
  }
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
