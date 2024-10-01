
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../../declarations/novelapp_backend";

const agent = new HttpAgent();
const novelAppBackend = Actor.createActor(idlFactory, {
  agent,
  canisterId: "xumeo-zyaaa-aaaab-qadaa-cai", // Replace with actual canister ID
});

export async function getNovels() {
  return await novelAppBackend.list_novels();
}

export async function getNovel(novelId) {
  return await novelAppBackend.get_novel(novelId);
}
