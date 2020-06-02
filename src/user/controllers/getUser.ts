import { createController } from "lib/createController";
import * as t from "io-ts";

const ID = t.type({ id: t.string });

export const getUser = createController({
  input: ID,
  async handler(h) {
    const { id } = h.input;
    return h.response(id);
  },
});
