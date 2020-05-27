import { createServer, IncomingMessage, ServerResponse } from "http";
import { UsersModule } from "./modules/users/users.controller";
import { Controller } from "./tools/controller";
import { ErrorsModule } from "./errors/errors.module";
import { Logger } from "./logger";

const server = createServer();

const modules: Record<string, typeof Controller> = {
  users: UsersModule,
};

const logger = new Logger("Main");

async function readRequest(req: IncomingMessage) {
  const buffer = [];
  return new Promise((resolve, reject) => {
    req.on("data", (chunk) => buffer.push(chunk));
    req.on("end", () => {
      try {
        resolve(JSON.parse(Buffer.concat(buffer).toString()));
      } catch (err) {
        resolve({});
      }
    });
    req.on("error", () => {
      reject(new Error("Request canceled"));
    });
  });
}

server.on("request", async (req: IncomingMessage, res: ServerResponse) => {
  const Module = modules[req.headers.module as string];
  const error = new ErrorsModule(req, res);
  if (!Module) {
    return error.NotFound(`Module ${req.headers.module} not found`);
  }
  const module = new Module(req, res);
  const handler = module[req.headers.controller as string];
  if (!handler) {
    return error.NotFound(`Controller ${req.headers.controller} not found`);
  }
  try {
    const body = await readRequest(req);
    const result = await handler.call(module, body);
    if (!res.finished) {
      module.send(result);
    }
  } catch (err) {
    logger.error(err);
    return error.ServerError();
  }
});

server.listen(8080);
