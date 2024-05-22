<script setup lang="ts">
import { ref, Ref } from "vue";
import init, { hello_world, WasmCamera } from '../../wasm/pkg/wasm'

init();

const refCanvas: Ref<HTMLCanvasElement | undefined> = ref();

const handleClick: () => void = (): void => {
  (async (): Promise<void> => {
    hello_world('world');

    const wasmCamera: WasmCamera = new WasmCamera();
    await wasmCamera.init();

    const canvas: HTMLCanvasElement = refCanvas.value!;
    const context: CanvasRenderingContext2D = canvas.getContext('2d')!;
    const mainloop = () => {
      wasmCamera.draw(canvas, context);
      window.requestAnimationFrame(mainloop);
    };
    mainloop();
  })().catch((error: unknown): void => { throw error; });
};
</script>

<template>
  <canvas ref="refCanvas" width="512" height="512" />
  <button @click="handleClick">start</button>
</template>

<style scoped>
</style>
