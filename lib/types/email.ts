import v from "validator";
import { t } from "../t";

const emailErr = "Invalid e-mail";
interface EmailBrand {
  readonly [emailErr]: symbol;
}
export const isEmail = t.brand(
  t.string,
  function (s): s is t.Branded<string, EmailBrand> {
    return v.isEmail(s);
  },
  emailErr
);
