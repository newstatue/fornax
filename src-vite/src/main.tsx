import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import { RouterProvider } from "react-router-dom";
import "./index.css"
import { ThemeProvider } from "@/components/theme-provider.tsx"
import {router} from "@/router/router.tsx";
import { Toaster } from "sonner"

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ThemeProvider>
      <Toaster richColors={true} position="top-center"/>
      <RouterProvider router={router}/>
    </ThemeProvider>
  </StrictMode>
)
