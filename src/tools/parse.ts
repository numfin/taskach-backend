import { Controller } from "src/tools/controller";
import { ErrorType } from "src/errors/ErrorType";
import { validate } from "class-validator";

export function Parse(c: new (...props: any[]) => any): MethodDecorator {
  return function (_, __, descriptor: PropertyDescriptor) {
    const fn: Function = descriptor.value;

    descriptor.value = async function (this: Controller, body) {
      const dto = new c();
      Object.assign(dto, body);
      const errors = await validate(dto);
      if (errors.length > 0) {
        return this.error(ErrorType.ValidationError, "Invalid request data", {
          fields: errors.reduce((acc, error) => {
            acc[error.property] = error.constraints;
            return acc;
          }, {}),
        });
      }
      return fn.apply(this, arguments);
    };
  };
}
