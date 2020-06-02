import { left, Either } from "fp-ts/lib/Either";

/** List of http error codes */
enum HttpCode {
  NotFound = 404,
  NotValid = 422,
  InternalError = 500,
}

/** List of http error messages */
const HttpMessages: Record<HttpCode, string> = {
  [HttpCode.NotFound]: "Not Found",
  [HttpCode.NotValid]: "Invalid Input",
  [HttpCode.InternalError]: "Internal Server Error",
};

type ErrCode = (code: typeof HttpCode) => HttpCode;
type CustomErrors = Record<string, string[]>;

export type HttpError = {
  code: HttpCode;
  message: string;
  errors?: CustomErrors;
};

export function createHttpError(err: {
  code: ErrCode;
  message?: string;
  errors?: CustomErrors;
}): Either<HttpError, never> {
  const code = err.code(HttpCode);
  const message = err.message ?? HttpMessages[code];
  return left({ code, message, errors: err.errors });
}
