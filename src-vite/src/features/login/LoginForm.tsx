import {GalleryVerticalEnd} from "lucide-react"
import { Link } from "react-router-dom";
import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import {
    Field,
    FieldDescription,
    FieldGroup,
    FieldLabel,
} from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import {InputOTP, InputOTPGroup, InputOTPSlot} from "@/components/ui/input-otp.tsx";
import {useState} from "react";
import * as React from "react";

export default function LoginForm() {
    const [step, setStep] = useState<"phone"|"code">("phone");
    const [isLoading, setIsLoading] = useState<boolean>(false);

    const handleSubmit = (e:React.SubmitEvent) => {
        e.preventDefault();
        setIsLoading(true);
        try {
            
            setStep("code");
        }catch (e) {
            setIsLoading(false);
        }

    }

    return (
        <div className={cn("flex flex-col gap-6")}>
            <form onSubmit={handleSubmit}>
                <FieldGroup>
                    <div className="flex flex-col items-center gap-2 text-center">
                        <a
                            href="#"
                            className="flex flex-col items-center gap-2 font-medium"
                        >
                            <div className="flex size-8 items-center justify-center rounded-md">
                                <GalleryVerticalEnd className="size-6" />
                            </div>
                            <span className="sr-only">天炉座</span>
                        </a>
                        <h1 className="text-xl font-bold">欢迎来到天炉座</h1>
                        <FieldDescription>
                            没有的账户会自动注册
                        </FieldDescription>
                    </div>

                    {step === "phone" && (
                        <Field>
                        <FieldLabel htmlFor="phone">电话</FieldLabel>
                        <Input
                            id="phone"
                            type="phone"
                            placeholder="12312345678"
                            required
                        />
                    </Field>
                    )}

                    {step === "code" && (
                        <div>
                            <Field>
                            <FieldLabel htmlFor="phone">验证码</FieldLabel>
                            <div className="flex gap-2">
                                <InputOTP maxLength={6} defaultValue="123456">
                                    <InputOTPGroup>
                                        <InputOTPSlot index={0} />
                                        <InputOTPSlot index={1} />
                                        <InputOTPSlot index={2} />
                                        <InputOTPSlot index={3} />
                                        <InputOTPSlot index={4} />
                                        <InputOTPSlot index={5} />
                                    </InputOTPGroup>
                                </InputOTP>
                                <Button>发送验证码</Button>
                            </div>
                        </Field>
                        </div>
                    )}

                    <Field>
                        <Button type="submit" disabled={isLoading} >下一步</Button>
                    </Field>

                </FieldGroup>
            </form>
            <FieldDescription className="px-6 text-center">
                点击继续，即表示您同意我们的
                <Link to="/terms">
                    《服务条款》
                </Link>
                {" "}和{" "}
                <Link to="/privacy">
                    《隐私政策》
                </Link>
                。
            </FieldDescription>
        </div>
    )
}
