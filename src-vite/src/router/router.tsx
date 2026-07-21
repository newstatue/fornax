import {createBrowserRouter} from "react-router-dom";
import AuthPage from "@/features/auth/AuthPage.tsx";
import RootPage from "@/RootPage.tsx";

export const router = createBrowserRouter([
    {
        path: "/auth",
        element: <AuthPage/>
    },
    {
        path:"/",
        element:<RootPage/>
    }
])