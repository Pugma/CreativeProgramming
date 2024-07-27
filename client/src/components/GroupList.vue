<script setup lang="ts">
import { type GroupItem } from '@/lib/apis'

interface Props {
  groups: GroupItem[]
}
defineProps<Props>()
</script>

<template>
  <ul>
    <li v-for="group in groups" :key="group.GroupUuid">
      {{ group.GroupName }}
    </li>
  </ul>
</template>

<style lang="scss" scoped></style>
