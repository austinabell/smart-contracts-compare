import { storage, context, ContractPromiseBatch } from "near-sdk-as";
import { values, ContentRecord } from "./model";

const OWNER_KEY = "owner";

// On contract initialization, set the owner key to the caller's address.
storage.set<string>(OWNER_KEY, context.predecessor);

// Purchase route, if enough value has been provided
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

// Return content data from a given route. This does not modify state.
export function getRoute(route: string): string | null {
  const value = values.get(route);
  return value ? value.content : null
}

// Withdraw funds from contract, if the caller is the contract owner.
export function withdraw(): void {
  let owner = storage.get<string>(OWNER_KEY);
  if (owner) {
    assert(context.predecessor, owner);
    ContractPromiseBatch.create(owner).transfer(context.accountBalance);
  }
}
