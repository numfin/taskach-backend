import { ObjectType, Field } from "@nestjs/graphql";
import { User } from "src/modules/Users/models/user";

@ObjectType()
export class Project {
  @Field()
  name: string;

  @Field()
  description: string;

  @Field(() => [User])
  projectUsers: User[];

  // @Field(() => [Story])
  // stories: Story[];
}
