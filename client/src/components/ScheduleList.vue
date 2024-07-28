<script setup lang="ts">
import { type ScheduleItem } from '@/lib/apis'

interface Props {
  schedules: ScheduleItem[]
}
defineProps<Props>()
</script>

<template>
  <ul>
    <li v-for="schedule in schedules" :key="schedule.UserName" :class="$style.item">
      <h5>{{ schedule.UserName }}</h5>
      {{ schedule.Since }} ~ {{ schedule.Until }}
      {{ schedule.Condition }}
    </li>
  </ul>
</template>

<style lang="scss" module>
ul {
  max-width: 80%;
}
li {
  list-style: none;
  padding: 4px;
  margin: 4px;
  border-radius: 4px;
  background-color: $color-primary-text-dark;
  align-content: center;
}

.item {
  color: $color-secondary-text-dark;
}
</style>
