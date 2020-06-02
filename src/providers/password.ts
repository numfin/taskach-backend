import { hash, compare } from "bcrypt";

export async function stringToHash(data: string) {
  return await hash(data, 12);
}

export async function compareHash(data: string, hashedString: string) {
  return await compare(data, hashedString);
}
