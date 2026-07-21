import request from "./axios"
import { getFornax } from "./generated"

export const api = getFornax(request)
