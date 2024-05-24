<script setup lang="ts">
import { ref } from 'vue'
import { AuthApi, type PostLogin, Configuration } from '@/lib/apis'

const apis = new AuthApi(new Configuration({ basePath: '/api' }))

const newUserName = ref('')
const newPassword = ref('')
const request = ref<PostLogin>({
  UserName: '',
  Password: ''
})

const postNewAccount = () => {
  request.value.UserName = newUserName.value
  request.value.Password = newPassword.value
  apis.signUpPost(request.value)
}
</script>

<template>
  <div>
    ユーザー名
    <input v-model="newUserName" @keypress.prevent.enter="postNewAccount" />
  </div>
  <div>
    パスワード
    <input v-model="newPassword" @keypress.prevent.enter="postNewAccount" />
  </div>
</template>
