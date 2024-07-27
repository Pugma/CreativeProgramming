/* eslint-disable no-restricted-imports */
import { AuthApi, Configuration, GroupApi, UserApi, ScheduleApi } from '@/lib/apis/generated'

class Apis {
  auth: AuthApi
  group: GroupApi
  user: UserApi
  schedule: ScheduleApi

  constructor() {
    this.auth = new AuthApi(new Configuration({ basePath: '/api' }))
    this.group = new GroupApi(new Configuration({ basePath: '/api' }))
    this.user = new UserApi(new Configuration({ basePath: '/api' }))
    this.schedule = new ScheduleApi(new Configuration({ basePath: '/api' }))
  }
}
const apis = new Apis()

export default apis
export * from '@/lib/apis/generated'
