import { Navigate, Outlet, useLocation } from "react-router-dom"
import { useAuthStore } from "@/store/auth.store.ts"


export default function AuthRouter() {
  const token = useAuthStore((state) => state.token)
  const location = useLocation()

  if (!token) {
    return <Navigate to="/auth" replace state={{ from: location }} />
  }

  return <Outlet />
}
