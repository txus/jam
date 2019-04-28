<template>
    <div class="subjam">
        <div class="controls">
            <div class="oscillators">
                <b-form-select :value="osc1_type" :options="osc_types" v-on:change="onOsc1TypeChange" class="osc-type"></b-form-select>
                <Knob v-on:change="onOscMixChange" :initial="osc_mix" v-bind:min=0 v-bind:max=1.0 label="Osc Mix" ringType='split'/>
                <b-form-select :value="osc2_type" :options="osc_types" v-on:change="onOsc2TypeChange" class="osc-type"></b-form-select>
            </div>
            <div class="vertical-knobs">
                <Knob v-on:change="onCutoffChange" :initial="cutoff" v-bind:min=0 v-bind:max=8000 label="Cutoff" ringType='negative'/>
                <Knob v-on:change="onResonanceChange" :initial="resonance" v-bind:min=0 v-bind:max=25.0 label="Resonance" ringType='positive'/>
            </div>
        </div>
        <div class="amp-envelope">
            <Envelope
            v-bind:adsr="env_adsr"
            v-on:onAttack="onAmpAttack"
            v-on:onDecay="onAmpDecay"
            v-on:onSustain="onAmpSustain"
            v-on:onRelease="onAmpRelease" />
        </div>
    </div>
</template>

<script>
import Envelope from './Envelope.vue'
import Knob from './Knob.vue'
export default {
    name: 'Subjam',
    props: ['subjam', 'rust'],
    components: { Knob, Envelope },
    data: function() {
        return {
            osc_types: [
                { value: 'sine', text: 'Sine'},
                { value: 'square', text: 'Square'},
                { value: 'sawtooth', text: 'Sawtooth'},
                { value: 'triangle', text: 'Triangle'},
                { value: 'custom', text: 'Custom', disabled: true},
            ]
            }
    },
    computed: {
        osc1_type: function() {
            if (this.subjam) {
                return this.subjam.get_osc1_type();
            } else {
                return 'sawtooth';
            }
        },
        osc2_type: function() {
            if (this.subjam) {
                return this.subjam.get_osc2_type();
            } else {
                return 'square';
            }
        },
        osc_mix: function() {
            if (this.subjam) {
                return this.subjam.osc_mix;
            } else {
                return 0.5;
            }
        },
        cutoff: function() {
            if (this.subjam) {
                return this.subjam.get_filter_frequency();
            } else {
                return 12000;
            }
        },
        resonance: function() {
            if (this.subjam) {
                return this.subjam.get_filter_resonance();
            } else {
                return 0.0;
            }
        },
        env_adsr: function() {
            if (this.subjam) {
                let env = this.subjam.get_osc1_env();
                return [env.attack, env.decay, env.sustain, env.release]
            } else {
                let env = this.rust.default_envelope();
                return [env.attack, env.decay, env.sustain, env.release]
            }
        },
    },
    methods: {
        onOsc1TypeChange: function(v) {
            if (this.subjam) {
                this.subjam.set_osc1_type(v);
            }
        },
        onOsc2TypeChange: function(v) {
            if (this.subjam) {
                this.subjam.set_osc2_type(v);
            }
        },
        onOscMixChange: function (v) {
            if (this.subjam) {
                console.log("Setting osc mix", v);
                this.subjam.set_osc_mix(v);
            }
        },
        onCutoffChange: function (v) {
            if (this.subjam) {
                console.log("Setting cuttoff", v);
                this.subjam.set_filter_frequency(v);
            }
        },
        onResonanceChange: function (v) {
            if (this.subjam) {
                console.log("Setting resonance", v);
                this.subjam.set_filter_resonance(v);
            }
        },
        onAmpAttack: function(v) {
            if (this.subjam) {
                this.subjam.set_env_attack(v);
            }
        },
        onAmpDecay: function(v) {
            if (this.subjam) {
                this.subjam.set_env_decay(v);
            }
        },
        onAmpSustain: function(v) {
            if (this.subjam) {
                this.subjam.set_env_sustain(v);
            }
        },
        onAmpRelease: function(v) {
            if (this.subjam) {
                this.subjam.set_env_release(v);
            }
        },
    }
}
</script>

<style lang="scss" scoped>
.subjam {
  margin: 0px auto;
  margin-bottom: 25px;
  padding: 20px;
  overflow: hidden;
  background: #363c40;
  border: 2px solid #272d31;
  box-shadow: 0 10px 70px rgba(#000000, 0.6);

  display: flex;
  justify-content: space-between;
  flex-direction: row;
  align-items: center;
}
.oscillators {
    display: flex;
    flex-direction: row;
    align-items: center;
    .osc-type {
        font-size: 70%;
        margin: -30px 10px 0px 10px;
    }
}
.vertical-knobs {
    display: flex;
    flex-direction: column;
}
.amp-envelope {
    right: 0;
}
</style>

