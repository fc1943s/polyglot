import * as fs from "fs"
import { NFTStorage } from "nft.storage"

const endpoint = "https://api.nft.storage"
const token = process.env.NFTSTORAGE_API_KEY || ""

async function main() {
  const storage = new NFTStorage({ endpoint: new URL(endpoint), token })

  const path = process.argv[2]
  const data = await fs.promises.readFile(path)
  const cidStore = await storage.storeBlob(new Blob([data]))
  console.log({ cidStore })
  await new Promise((resolve) => setTimeout(resolve, 4000))
  const status = await storage.status(cidStore)
  console.log({ status })
}
await main()
