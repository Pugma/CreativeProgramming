<script setup lang="ts">
import apis from '@/lib/apis'

const userName = (await apis.user.meGet()).data.UserName
</script>

<template>
  <header>
    <h1>Scheduling</h1>
    <div>{{ userName }}</div>
  </header>
</template>

<style scoped lang="scss">
header {
  width: 100vw;
  height: 5rem;
  display: flex;
  padding: 8px;
  background-color: #aaaaaa;
  align-items: center;
  justify-content: space-between;

  div {
    max-width: 40%;
    overflow: hidden;
    white-space: nowrap;
  }
}
</style>
