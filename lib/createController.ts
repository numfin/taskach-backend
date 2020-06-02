import * as t from "io-ts";
import { Either, isRight, right } from "fp-ts/lib/Either";
import { createHttpError, HttpError } from "./httpError";
import { IncomingMessage } from "http";
import { parseRequest } from "./parseRequest";

function createParser<T>(type: t.Type<T>): (data: T) => Either<HttpError, T> {
  return (data: T) => {
    const decoded = type.decode(data);
    if (isRight(decoded)) {
      return right(decoded.right);
    }
    const errors = decoded.left.reduce((acc, { context }) => {
      const { type } = context[context.length - 1];
      const key = context.flatMap(({ key }) => (key ? [key] : [])).join(".");
      const message = type.name;
      acc[key] = [message];
      return acc;
    }, {} as Record<string, string[]>);
    return createHttpError({ code: (c) => c.NotValid, errors });
  };
}

interface Handler<T> {
  input: T;
  error: typeof createHttpError;
  response: typeof right;
}

export type Controller = (
  req: IncomingMessage
) => Promise<Either<HttpError, unknown>>;

export type ControllerHandler<T> = (
  h: Handler<T>
) => Promise<Either<HttpError, unknown>>;

interface ControllerParams<Input> {
  input: t.Type<Input>;
  handler: ControllerHandler<Input>;
}

export function createController<Input>({
  input,
  handler,
}: ControllerParams<Input>): Controller {
  const parseData = createParser(input);

  return async function Controller(req: IncomingMessage) {
    try {
      const data = await parseRequest(req);
      const eitherInput = parseData(data);
      if (isRight(eitherInput)) {
        return handler({
          input: eitherInput.right,
          error: createHttpError,
          response: right,
        });
      }
      return eitherInput;
    } catch (err) {
      return createHttpError({ code: (c) => c.InternalError });
    }
  };
}
