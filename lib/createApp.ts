import { createServer } from "http";
import { isRight } from "fp-ts/lib/Either";
import { getController, ControllerMap } from "./createModule";

interface AppOptions {
  modules: Array<(map: ControllerMap) => void>;
}

export function createApp(appOptions: AppOptions) {
  const map: ControllerMap = {};
  for (const initModule of appOptions.modules) {
    initModule(map);
  }

  const server = createServer(async (req, res) => {
    const { module, controller } = req.headers;
    const c = getController(module as string, controller as string, map);

    const resultOrError = await c(req);
    if (isRight(resultOrError)) {
      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(JSON.stringify(resultOrError.right));
    } else {
      const { code, message, errors } = resultOrError.left;
      res.writeHead(code, message, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ message, errors }));
    }
  });
  return { server };
}
