import * as t from "io-ts";
import { Either, isRight, right } from "fp-ts/lib/Either";
import { createHttpError, HttpError } from "./httpError";
import { IncomingMessage } from "http";
import { parseRequest } from "./parseRequest";
import { validate, validateOrReject } from "class-validator";

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
  return async function Controller(req: IncomingMessage) {
    try {
      const data = await parseRequest(req);
      try {
        const dataValidated = await validateOrReject(data);

      }
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
