<template>
    <div class="knob">
        <div ref="control" class="knob__control envelope-knob"></div>
        <div class="knob__label">
            {{label}}
        </div>
    </div>
</template>

<script>
import * as PrecisionInputs from 'precision-inputs/common/precision-inputs.base.js'
import * as FLPrecisionInputs from 'precision-inputs/common/precision-inputs.fl-controls.js'

export default {
  name: 'Knob',
  props: {
      initial: { type: Number, required: true },
      label: { type: String, required: true },
      min: { type: Number, required: true },
      max: { type: Number, required: true },
      ringType: { type: String, required: true },
  },
  data: function() {
      return {
        knob: null,
        value: this.initial,
      }
  },
  mounted() {
      let control = this.$refs.control;
      this.knob = new FLPrecisionInputs.FLStandardKnob(control, {
          min: this.min, max: this.max, initial: this.initial, indicatorRingType: this.ringType
          });
      this.knob.addEventListener('change', (evt) => {
          this.value = parseFloat(evt.target.value);
          this.$emit("change", this.value);
      })
  }
}
</script>

<style lang="scss" scoped>
$envelope-knob-size: 80px;
$tension-knob-size: 60px;
.knob {
  flex: 1 0 auto;
  display: flex;
  flex-direction: column;
  align-items: center;
}
.knob__label {
  flex: 0 0 auto;
  display: inline-block;
  padding: 10px 0;
  width: $envelope-knob-size;
  color: #c1c5c5;
  font-size: 10px;
  font-family: 'Helvetica', sans-serif;
  font-weight: 700;
  text-align: center;
  letter-spacing: 1px;
  text-transform: uppercase;
}

// FL Standard Knobs - sizing for wrapper class demo
.knob__control {
  flex: 0 0 auto;
  &.envelope-knob {
    width: $envelope-knob-size;
    height: $envelope-knob-size;
  }
  &.tension-knob {
    padding: ($envelope-knob-size - $tension-knob-size)/2;
    width: $envelope-knob-size;
    height: $envelope-knob-size;
  }
}
</style>