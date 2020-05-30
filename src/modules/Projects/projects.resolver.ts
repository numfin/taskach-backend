import { Resolver } from "@nestjs/graphql";
import { Project } from "./models/project";

@Resolver(() => Project)
export class ProjectResolver {}
