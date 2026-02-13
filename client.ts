import { Connection, PublicKey } from "@solana/web3.js";

export class AureonClient {
  constructor(public connection: Connection) {}

  async getReserve(vault: PublicKey) {
    return this.connection.getAccountInfo(vault);
  }
}
