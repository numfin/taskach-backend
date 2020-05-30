import { ArgsType, Field } from "@nestjs/graphql";
import { MinLength } from "class-validator";

@ArgsType()
export class CreateUserArg {
  @Field()
  @MinLength(3)
  name: string;

  @Field()
  @MinLength(3)
  surname: string;
}
