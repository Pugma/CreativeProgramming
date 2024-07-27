<script setup lang="ts">
import { type ScheduleItem } from '@/lib/apis'

interface Props {
  schedules: ScheduleItem[]
}
defineProps<Props>()
</script>

<template>
  <ul>
    <li v-for="schedule in schedules" :key="schedule.UserName">
      {{ schedule.Since }} ~ {{ schedule.Until }}
      {{ schedule.Condition }}
    </li>
  </ul>
</template>

<style lang="scss" scoped></style>
