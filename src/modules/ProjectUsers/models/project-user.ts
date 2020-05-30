import { ObjectType, Field } from "@nestjs/graphql";
import { User } from "src/modules/Users/models/user";

@ObjectType()
export class ProjectUser {
  @Field(() => User)
  user: User;

  // @Field(() => Position)
  // position: Position

  // @Field(() => [Task])
  // activeTask: [Task]

  // @Field(() => )
}
