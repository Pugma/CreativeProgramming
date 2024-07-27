<script setup lang="ts">
import PageHeader from '@/components/PageHeader.vue'
import PageContainer from '@/components/PageContainer.vue'
import apis, { type ScheduleItem } from '@/lib/apis'
import { ref } from 'vue'
import PageTitle from '@/components/PageTitle.vue'
import useParam from '@/lib/param'
import ScheduleList from '@/components/ScheduleList.vue'

const info = ref<ScheduleItem[]>([])
const groupId = useParam('groupId')

info.value = (await apis.schedule.scheduleGroupIdGet(groupId.value)).data
</script>

<template>
  <PageHeader />
  <PageContainer>
    <PageTitle title="メンバーの忙しさ一覧" />
    <ScheduleList :schedules="info" />
  </PageContainer>
</template>

<style lang="scss" scoped></style>
