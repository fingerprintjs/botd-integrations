import {Status} from "./constants";

export type HeadersDict = Record<string, unknown>

export interface DetectResultItem {
    status: string
    prob: number
    type: string
}

export interface DetectResultError {
    status: Status.ERROR
    error: string
}