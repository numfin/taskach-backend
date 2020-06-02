import { t } from "../t";

interface MaxLengthBrand {
  readonly [any: string]: symbol;
}
export function maxLength(len: number) {
  const err = `Should be less than ${len} symbols`;

  return t.brand(
    t.string,
    function (s): s is t.Branded<string, MaxLengthBrand> {
      return s.trim().length < len;
    },
    err
  );
}
