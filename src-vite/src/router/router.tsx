import {createBrowserRouter} from "react-router-dom";
import LoginPage from "@/features/login/LoginPage.tsx";
import RootPage from "@/RootPage.tsx";

export const router = createBrowserRouter([
    {
        path: "/login",
        element: <LoginPage/>
    },
    {
        path:"/",
        element:<RootPage/>
    }
])