import { Resolver } from "@nestjs/graphql";
import { ProjectUser } from "./models/project-user";

@Resolver(() => ProjectUser)
export class ProjectUsersResolver {}
