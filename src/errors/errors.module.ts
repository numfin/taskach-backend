import { Controller } from "src/tools/controller";
import { ErrorType } from "src/errors/ErrorType";

export class ErrorsModule extends Controller {
  async NotFound(message = "Not Found") {
    this.error(ErrorType.NotFound, message);
  }

  async ServerError() {
    this.error(ErrorType.ServerError, "Internal Server Error");
  }
}
