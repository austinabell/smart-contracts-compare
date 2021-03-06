import { storage, logging, env, context } from "near-sdk-as";
import { values, ContentRecord } from "./model";

// --- contract code goes below

const OWNER_KEY = "owner";

export function init(): void {
  storage.set<string>(OWNER_KEY, context.predecessor);
}

export function incrementCounter(value: i32): void {
  
}

export function purchase(route: string, content: string): void {

}

export function getRoute(route: string): string | null {
  const value = values.get(route);
  return value ? value.content : null
}

export function withdraw(): void {
  
}
