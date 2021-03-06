import { context, u128, PersistentUnorderedMap, Storage } from "near-sdk-as";

/** 
 * Exporting a new class ContentRecord so it can be used outside of this file.
 */
@nearBindgen
export class ContentRecord {
    public price: u128;
    public content: string;
    public owner: string;
}

// Stores the content mapping for routes to content and associated metadata.
export const values = new PersistentUnorderedMap<string, ContentRecord>('v');
