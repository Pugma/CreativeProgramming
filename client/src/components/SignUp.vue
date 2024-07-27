<script setup lang="ts">
import { ref } from 'vue'
import apis, { type PostLogin } from '@/lib/apis'
import PageContainer from '@/components/PageContainer.vue'
import PageTitle from '@/components/PageTitle.vue'

const newUserName = ref('')
const newPassword = ref('')
const request = ref<PostLogin>({
  UserName: '',
  Password: ''
})

const postNewAccount = () => {
  request.value.UserName = newUserName.value
  request.value.Password = newPassword.value
  apis.auth.signUpPost(request.value)
}
</script>

<template>
  <PageContainer>
    <PageTitle title="サインアップ" />
    <div>
      ユーザー名
      <input v-model="newUserName" @keypress.prevent.enter="postNewAccount" />
    </div>
    <div>
      パスワード
      <input type="password" v-model="newPassword" @keypress.prevent.enter="postNewAccount" />
    </div>
    <button @click="postNewAccount">サインアップ</button>
  </PageContainer>
</template>
