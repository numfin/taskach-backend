import { IncomingMessage, ServerResponse } from "http";
import { ErrorType } from "src/errors/ErrorType";
import { datastore } from "src/providers/firestore";

function replacer(_: any, value: any) {
  const key = value?.[datastore.KEY];
  if (key) {
    return {
      id: key.name || Number(key.id),
      ...value,
    };
  }
  return value;
}

export class Controller {
  private defaultHeaders = {
    "Content-Type": "application/json",
  };
  headers: Record<string, string> = new Proxy(this.defaultHeaders, {
    set(target, key: string, value) {
      if (key in target) {
        throw new Error(`Cannot write existing headers: ${key}`);
      }
      target[key] = value;
      return true;
    },
  });

  constructor(
    public request: IncomingMessage,
    public response: ServerResponse
  ) {}

  error(statusCode: ErrorType, message?: string, additionalData = {}) {
    this.send(
      {
        message,
        ...additionalData,
      },
      { statusCode }
    );
  }

  send(data?: unknown, options?: { statusCode?: number }) {
    this.response.writeHead(options?.statusCode ?? 200, this.headers);

    this.response.end(
      data !== undefined ? JSON.stringify(data, replacer) : data
    );
  }
}
