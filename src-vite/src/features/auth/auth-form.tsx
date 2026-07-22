import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import {
  Field,
  FieldDescription,
  FieldGroup,
  FieldLabel,
} from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import {
  InputOTP,
  InputOTPGroup,
  InputOTPSlot,
} from "@/components/ui/input-otp.tsx"
import { useEffect, useState } from "react"
import { REGEXP_ONLY_DIGITS } from "input-otp"
import * as React from "react"
import { api } from "@/api"
import { toast } from "sonner"
import { useAuthStore } from "@/store/auth.store.ts"

type AuthFormProps = {
  onSuccess?: () => void
}

export function AuthForm({ onSuccess }: AuthFormProps) {
  const [email, setEmail] = useState("")
  const [code, setCode] = useState("")
  const [isSendingCode, setIsSendingCode] = useState(false)
  const [isLoggingIn, setIsLoggingIn] = useState(false)
  const [cd,setCd] = useState(0)

  const codeCompleted = code.length === 5

  useEffect(() => {
    if (cd <= 0){
      return
    }

    const timer = setTimeout(() => {
      setCd((current)=> current - 1)
    },1000)

    return () => window.clearTimeout(timer)
  },[cd])

  const handleSendCode = async () => {
    setIsSendingCode(true)

    try {
      const { data } = await api.sendCode({
        email: email,
      })
      toast.success(data.msg ?? "验证码发送成功")

      setCd(data.data?.cd ?? 60)

    } catch (error) {
      console.error("发送验证码失败：", error)
    } finally {
      setIsSendingCode(false)
    }
  }

  const handleSubmit = async (event: React.SubmitEvent<HTMLFormElement>) => {
    event.preventDefault()
    setIsLoggingIn(true)

    try {
      const { data } = await api.login({
        email,
        code,
      })

      const token = data.data?.token

      if (!token) {
        console.error("登录失败，响应中不包含token")
        toast.error("登录成功，但未返回 Token")
        return
      }

      useAuthStore.getState().setToken(token)

      toast.success(data.msg ?? "登录成功")
      onSuccess?.()
    } catch (error) {
      console.error("登录失败：", error)
    } finally {
      setIsLoggingIn(false)
    }
  }

  return (
    <div className="flex flex-col gap-6">
      <Card>
        <CardHeader>
          <CardTitle>登录到您的账户</CardTitle>
          <CardDescription>在下方输入电子邮箱以登录您的账户</CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit}>
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="email">电子邮箱</FieldLabel>
                <Input
                  id="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  type="email"
                  placeholder="m@example.com"
                  required
                />
              </Field>
              <Field>
                <FieldLabel htmlFor="code">验证码</FieldLabel>
                <div className="flex flex-wrap justify-between gap-3">
                  <InputOTP
                    maxLength={5}
                    defaultValue=""
                    value={code}
                    onChange={setCode}
                    pattern={REGEXP_ONLY_DIGITS}
                    disabled={isLoggingIn}
                    required
                  >
                    <InputOTPGroup>
                      <InputOTPSlot index={0} />
                      <InputOTPSlot index={1} />
                      <InputOTPSlot index={2} />
                      <InputOTPSlot index={3} />
                      <InputOTPSlot index={4} />
                    </InputOTPGroup>
                  </InputOTP>
                  <Button
                    className="flex-1"
                    type="button"
                    disabled={isSendingCode || !email || cd > 0}
                    onClick={handleSendCode}
                  >
                    {isSendingCode ? "发送中..." : cd > 0 ? `${cd}秒后重试`: "发送验证码"}
                  </Button>
                </div>
              </Field>
              <Field>
                <Button type="submit" disabled={!codeCompleted || !email}>
                  登录
                </Button>
                <FieldDescription className="text-center">
                  没有帐户会自动注册
                </FieldDescription>
              </Field>
            </FieldGroup>
          </form>
        </CardContent>
      </Card>
    </div>
  )
}
