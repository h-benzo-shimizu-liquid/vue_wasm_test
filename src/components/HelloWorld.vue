<script setup lang="ts">
import init, { greet } from '../../wasm/pkg/wasm'

init();

const handleGreet: () => void = (): void => {
  greet('test');
};
</script>

<template>
  <button @click="handleGreet">greet</button>
</template>

<style scoped>
</style>
