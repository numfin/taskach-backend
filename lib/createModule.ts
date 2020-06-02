import { Controller } from "./createController";
import { ErrorController } from "./error.controller";

export type ControllerMap = Record<string, Record<string, Controller>>;

export function createModule<C extends Record<string, Controller>>(
  module: string,
  controllers: C
) {
  return function initModule(map: ControllerMap): void {
    map[module] = controllers;
  };
}

export function getController(
  module: string,
  controller: string,
  map: ControllerMap
): Controller {
  return map[module]?.[controller] ?? ErrorController;
}
