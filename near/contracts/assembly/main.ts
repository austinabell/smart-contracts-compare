import { storage, logging, env, context, ContractPromiseBatch } from "near-sdk-as";
import { values, ContentRecord } from "./model";

// --- contract code goes below

const OWNER_KEY = "owner";

// TODO should not require calling a function to initialize state, look into lifecycle (no examples)
export function init(): void {
  assert(!storage.hasKey(OWNER_KEY));
  storage.set<string>(OWNER_KEY, context.predecessor);
}

export function purchase(route: string, content: string): void {
  const deposit = context.attachedDeposit;
  assert(!deposit.isZero(), "Deposit required to purchase route");
  const existing = values.get(route);
  if (existing) {
    assert(deposit > existing.price, "Not enough deposit to purchase route")

    ContractPromiseBatch.create(existing.owner).transfer(existing.price);
  }

  let rec: ContentRecord = { price: deposit, content, owner: context.predecessor };
  values.set(route, rec)
}

export function getRoute(route: string): string | null {
  const value = values.get(route);
  return value ? value.content : null
}

export function withdraw(): void {
  let owner = storage.get<string>(OWNER_KEY);
  if (owner) {
    assert(context.predecessor, owner);
    ContractPromiseBatch.create(owner).transfer(context.accountBalance);
  }
}
