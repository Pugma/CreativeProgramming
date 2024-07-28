<script setup lang="ts">
interface Props {
  title: string
}

defineProps<Props>()
</script>

<template>
  <div class="title">
    <h1>{{ title }}</h1>
  </div>
</template>

<style scoped lang="scss">
.title {
  height: 5rem;
}
</style>
