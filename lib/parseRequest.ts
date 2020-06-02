import { IncomingMessage } from "http";

export async function parseRequest(req: IncomingMessage): Promise<any> {
  const buffer: Uint8Array[] = [];
  let isFinished = false;

  return new Promise((resolve, reject) => {
    req.on("data", (part) => buffer.push(part));
    req.on("end", () => {
      isFinished = true;
      try {
        resolve(JSON.parse(Buffer.concat(buffer).toString()));
      } catch (err) {
        reject(err);
      }
    });
    req.on("close", () => {
      if (!isFinished) {
        reject(new Error("Connection reset"));
      }
    });
    req.on("error", (err) => reject(err));
  });
}
