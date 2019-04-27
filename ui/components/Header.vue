<template>
  <div class="header">
    <h1>{{ msg }}</h1>
    <b-form-checkbox
      id="power"
      v-model="power"
      name="power"
      value="ON"
      unchecked-value="OFF"
      v-on:change="switch_power"
      switch>
      Power: {{ power }}
      </b-form-checkbox>
  </div>
</template>

<script>
export default {
  name: 'Header',
  data () {
    return {
      msg: 'Subjam',
      power: 'OFF'
    }
  },
  methods: {
    switch_power: function (v) {
      this.$emit('power', v == 'ON');
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1, h2 {
  font-weight: normal;
}
</style>
