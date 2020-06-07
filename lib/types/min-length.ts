import { t } from "../t";

interface MinLengthBrand {
  readonly [any: string]: symbol;
}
export function minLength(len: number) {
  const err = `Should be longer than ${len} symbols`;

  return t.union([
    t.string,
    t.brand(
      t.string,
      function (s): s is t.Branded<string, MinLengthBrand> {
        return s.trim().length > len;
      },
      err
    ),
  ]);
}
