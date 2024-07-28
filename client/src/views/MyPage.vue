<script setup lang="ts">
import PageHeader from '@/components/PageHeader.vue'
import PageContainer from '@/components/PageContainer.vue'
import apis, { type GroupItem } from '@/lib/apis'
import { ref } from 'vue'
import GroupList from '@/components/GroupList.vue'
import PageTitle from '@/components/PageTitle.vue'

const groups = ref<GroupItem[]>([])
groups.value = (await apis.group.groupGet()).data
</script>

<template>
  <PageHeader />
  <PageContainer>
    <PageTitle title="所属グループ一覧" />
    <GroupList :groups="groups" />
  </PageContainer>
</template>

<style lang="scss" scoped></style>
