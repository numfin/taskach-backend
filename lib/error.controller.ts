import * as t from "io-ts";
import { createController } from "./createController";

export const ErrorController = createController({
  input: t.type({}),
  async handler(h) {
    return h.error({
      code: (e) => e.NotFound,
      message: "Module or Controller Not Found",
    });
  },
});
