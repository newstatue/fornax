import axios from "axios"
import { toast } from "sonner"

interface ApiErrorResponse {
  code: number
  msg: string
  errors?: Record<string, string[]>
}

const request = axios.create({
  baseURL: "",
  timeout: 10000,
})

request.interceptors.request.use((config) => {
  const token = localStorage.getItem("token")

  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }

  return config
})

request.interceptors.response.use(
  (response) => response,

  (error: unknown) => {
    if (axios.isAxiosError<ApiErrorResponse>(error)) {
      const data = error.response?.data

      if (!error.response) {
        toast.error("网络连接失败")
      } else if (data?.errors) {
        Object.values(data.errors)
          .flat()
          .forEach((message) => {
            toast.error(message)
          })
      } else if (data?.msg) {
        toast.error(data.msg)
      } else {
        toast.error(`请求失败 (${error.response.status})`)
      }
    } else {
      toast.error("发生未知错误")
    }

    return Promise.reject(error)
  }
)

export default request
