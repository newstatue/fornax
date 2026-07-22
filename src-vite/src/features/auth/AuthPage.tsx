import { useNavigate } from "react-router-dom"
import { AuthForm } from "@/features/auth/auth-form"

export default function AuthPage() {
  const navigate = useNavigate()

  return (
    <div className="flex min-h-svh w-full items-center justify-center p-6 md:p-10">
      <div className="w-full max-w-sm">
        <AuthForm onSuccess={() => navigate("/")} />
      </div>
    </div>
  )
}
