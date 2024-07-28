<script setup lang="ts">
import apis, { type ScheduleItem } from '@/lib/apis'
import useParam from '@/lib/param'
import { ref } from 'vue'
import PageTitle from '@/components/PageTitle.vue'

const groupId = useParam('groupId')
const newSchedule = ref<ScheduleItem>({
  UserName: '',
  Since: '',
  Until: '',
  Condition: 0
})

apis.schedule.scheduleGroupIdPost(groupId.value, newSchedule.value)
</script>

<template>
  <section>
    <PageTitle title="自分の忙しさを登録する" />
    <input type="date" :class="$style.date" /> ~ <input type="date" :class="$style.date" />
    <div>
      <input type="radio" id="0" :value="0" v-model="newSchedule.Condition" />
      <label for="0">めちゃ余裕</label>
      <input type="radio" id="1" :value="1" v-model="newSchedule.Condition" />
      <label for="1">余裕がなくはない</label>
      <input type="radio" id="2" :value="2" v-model="newSchedule.Condition" />
      <label for="2">どちらでもない</label>
      <input type="radio" id="3" :value="3" v-model="newSchedule.Condition" />
      <label for="3">そこまでゆとりがない</label
      ><input type="radio" id="3" :value="3" v-model="newSchedule.Condition" />
      <label for="3">マジでゆとりがない</label>
    </div>
    <div>
      {{ newSchedule.Condition }}
    </div>
  </section>
</template>

<style lang="scss" module>
section {
  padding: 8px;
}
.date {
  font-size: 1.2rem;
  width: 16rem;
}
</style>
