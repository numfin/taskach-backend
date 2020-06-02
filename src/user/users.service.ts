import { datastore } from "src/providers/datastore";
import cuid from "cuid";
import { stringToHash } from "src/providers/password";

interface User {
  name: string;
  email: string;
  password: string;
}

async function create(user: User): Promise<string> {
  const id = cuid();
  const hash = await stringToHash(user.password);
  delete user.password;

  await datastore.insert({
    key: datastore.key("users"),
    data: { ...user, hash, id },
  });
  return id;
}

export const user = {
  create,
};
