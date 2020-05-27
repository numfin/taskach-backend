const mem = new WeakMap();

function construct(constructor: Function, args: any[]) {
  if (!mem.has(constructor)) {
    const nameObj = {
      [constructor.name]: function (this: any) {
        return constructor.apply(this, args);
      },
    };
    nameObj[constructor.name].prototype = constructor.prototype;
    mem.set(constructor, new (nameObj[constructor.name] as any)());
  }
  return mem.get(constructor);
}

export function Singleton(): ClassDecorator {
  return function classDecorator(target) {
    const f: any = function (...args: any[]) {
      return construct(target, args);
    };
    f.prototype = target.prototype;
    return f;
  };
}
