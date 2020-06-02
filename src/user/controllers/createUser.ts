import { createController } from "lib/createController";
import { user } from "../users.service";
import { t } from "lib/t";
import { isEmail, minLength, maxLength } from "lib/types";

const User = t.type({
  name: t.intersection([minLength(5), maxLength(32)]),
  email: isEmail,
  password: t.intersection([minLength(8), maxLength(64)]),
});

export const createUser = createController({
  input: User,
  async handler(h) {
    return h.response(h.input);
    // try {
    //   const id = await user.create(h.input);
    //   return h.response(id);
    // } catch (e) {
    //   console.trace(e);
    //   return h.error({ code: (err) => err.InternalError });
    // }
  },
});
