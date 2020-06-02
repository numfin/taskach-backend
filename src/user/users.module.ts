import { createModule } from "lib/createModule";

import { getUser } from "./controllers/getUser";
import { createUser } from "./controllers/createUser";

export const UserModule = createModule("user", {
  getUser,
  createUser,
});
