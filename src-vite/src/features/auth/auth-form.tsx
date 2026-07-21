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
import { useState } from "react"
import { REGEXP_ONLY_DIGITS } from "input-otp"
import * as React from "react"
import { api } from "@/api"
import { toast } from "sonner"

export function AuthForm() {
  const [email, setEmail] = useState("")
  const [code, setCode] = useState("")
  const [isSendingCode, setIsSendingCode] = useState(false)
  const [isLoggingIn, setIsLoggingIn] = useState(false)

  const codeCompleted = code.length === 6
  const handleSendCode = async () => {
    setIsSendingCode(true)

    try {
      const { data } = await api.sendCode({
        email: email,
      })

      setEmail(email)
      toast.success(data.msg ?? "验证码发送成功")
    } catch (error) {
      console.error("发送验证码失败：", error)
      toast.error("验证码发送失败，请稍后重试")
    } finally {
      setIsSendingCode(false)
    }
  }

  const handleSubmit = async (
    event: React.SyntheticEvent<HTMLFormElement, SubmitEvent>
  ) => {
    event.preventDefault()
    setIsLoggingIn(true)

    try {
      const { data } = await api.login({
        email: email,
        code,
      })

      localStorage.setItem("token", data.token)

      toast.success("登录成功")

    } catch (error) {
      console.error("登录失败：", error)
      toast.error("登录失败，请检查邮箱和验证码")
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
                <div className="flex justify-between">
                  <InputOTP
                    maxLength={6}
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
                      <InputOTPSlot index={5} />
                    </InputOTPGroup>
                  </InputOTP>
                  <Button type="button"
                          disabled={isSendingCode}
                          onClick={handleSendCode}>发送验证码</Button>
                </div>
              </Field>
              <Field>
                <Button type="submit" disabled={!codeCompleted}>登录</Button>
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
