<script setup lang="ts">
import PageHeader from '@/components/PageHeader.vue'
import PageContainer from '@/components/PageContainer.vue'
import SignUp from '@/components/SignUp.vue'
</script>

<template>
  <PageHeader title="トップページ" />
  <PageContainer>
    このサイトは、予定管理を簡単に行うためのものです
    <sign-up />
  </PageContainer>
</template>

<style lang="scss" scoped>
// * {
//   color: $color-primary;
// }
</style>
