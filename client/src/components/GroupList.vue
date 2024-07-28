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
      <router-link
        :to="{ name: 'GroupPage', params: { groupId: group.GroupUuid } }"
        :class="$style.item"
      >
        <h5>{{ group.GroupName }}</h5>
      </router-link>
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
