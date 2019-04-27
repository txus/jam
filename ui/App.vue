<template>
  <div id="app">
    <Header v-on:power="onPower"/>
    <hr/>
    <div class="mixer">
       <Knob v-on:change="onMix" :initial="osc_mix" v-bind:min=0 v-bind:max=1.0 label="Osc Mix" ringType='split'/>
    </div>
    <Envelope
      v-bind:adsr="amp_adsr"
      v-on:onAttack="onAmpAttack"
      v-on:onDecay="onAmpDecay"
      v-on:onSustain="onAmpSustain"
      v-on:onRelease="onAmpRelease" />
  </div>
</template>

<script>
import Header from './components/Header.vue'
import Envelope from './components/Envelope.vue'
import Knob from './components/Knob.vue'
export default {
  name: 'App',
  props: ['rust'],
  components: {
    Header,
    Knob,
    Envelope
  },
  computed: {
    osc_mix: function() {
      if (this.synth) {
        return this.synth.osc_mix()
      } else {
        return 0.5
      }
    },
    amp_adsr: function() {
      if (this.synth) {
        return this.synth.adsr()
      } else {
        return [0.1, 0.0, 1.0, 0.8]
      }
    }
  },
  methods: {
    noteOn: function(note, velocity) {
      console.log("NOTE ON!", note, velocity);
      if (this.synth) {
        this.synth.note_on(note, velocity);
      }
    },
    noteOff: function(note) {
      console.log("NOTE OFF!", note);
      if (this.synth) {
        this.synth.note_off(note);
      }
    },
    onPower: function(is_on) {
      if (is_on) {
        this.audioContext = new AudioContext();
        this.synth = new this.rust.Subjam(this.audioContext);
      } else {
        this.synth.free();
        this.synth = null;
        this.audioContext = null;
      }
    },
    onAmpAttack: function(v) {
      if (this.synth) { this.synth.set_env_attack(v); }
    },
    onAmpDecay: function(v) {
      if (this.synth) { this.synth.set_env_decay(v); }
    },
    onAmpSustain: function(v) {
      if (this.synth) { this.synth.set_env_sustain(v); }
    },
    onAmpRelease: function(v) {
      if (this.synth) { this.synth.set_env_release(v); }
    },
    onMix: function(v) {
      console.log(this.synth);
      if (this.synth) { this.synth.set_amp_mix(v); }
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

.mixer {
  margin: 0px auto;
  width: 150px;
  height: 150px;
  margin-bottom: 25px;
  padding: 20px;
  overflow: hidden;
  background: #363c40;
  border: 2px solid #272d31;
  box-shadow: 0 10px 70px rgba(#000000, 0.6);

  display: flex;
  justify-content: space-between;
  align-items: center;
}

</style>
