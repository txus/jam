import './style.scss';
import 'precision-inputs/css/precision-inputs.base.css';
import 'precision-inputs/css/precision-inputs.fl-controls.css';

import Vue from 'vue'
import BootstrapVue from 'bootstrap-vue';
import App from './App.vue'

Vue.use(BootstrapVue);

import('./../pkg/jam')
  .then(rust_module => {
    let vue = new Vue({
      el: '#app',
      props: ['rust'],
      components: { App },
      render: h => h(App, {props: {rust: rust_module}})
    });

    // MIDI

    if (navigator.requestMIDIAccess) {
        console.log('This browser supports WebMIDI!');
    } else {
        console.error('WebMIDI is not supported in this browser.');
    }

    let midiInputs;

    const onMIDIMessage = ({data}) => {
      let cmd = data[0] >> 4;
      let channel = data[0] & 0xf;
      let type = data[0] & 0xf0;
      let note = data[1];
      let velocity = data[2];

      switch (type) {
        case 144:
          vue.$children[0].noteOn(note, velocity);
          break;
        case 128:
          vue.$children[0].noteOff(note, velocity);
          break;
      }
    };

    navigator.requestMIDIAccess()
    .then(
      midiAccess => {
        midiInputs = midiAccess.inputs.values();
        for (var input = midiInputs.next(); input && !input.done; input = midiInputs.next()) {
          // each time there is a midi message call the onMIDIMessage function
          input.value.onmidimessage = onMIDIMessage;
      }
      },
      () => console.error('Could not access your MIDI devices')
      );
  })
  .catch(console.error);
