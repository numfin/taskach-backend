import { ObjectType, Field } from "@nestjs/graphql";
import { Project } from "src/modules/Projects/models/project";

@ObjectType()
export class User {
  @Field()
  id: string;

  @Field(() => String)
  name: string;
  @Field(() => String)
  surname: String;

  @Field(() => [Project])
  projects: Project[];
}
