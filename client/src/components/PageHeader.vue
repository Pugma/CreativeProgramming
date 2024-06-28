<script setup lang="ts">
interface Props {
  title: string
}

defineProps<Props>()
</script>

<template>
  <header>
    <h1>{{ title }}</h1>
  </header>
</template>

<style scoped lang="scss">
header {
  width: 100vw;
  height: 5rem;
}
</style>
