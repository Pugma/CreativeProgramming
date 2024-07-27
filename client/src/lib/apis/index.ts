/* eslint-disable no-restricted-imports */
import { AuthApi, Configuration } from '@/lib/apis/generated'

const apis = new AuthApi(new Configuration({ basePath: '/api' }))

export default apis
export * from '@/lib/apis/generated'
