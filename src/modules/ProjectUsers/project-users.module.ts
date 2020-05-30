import { Module } from "@nestjs/common";
import { ProjectUsersResolver } from "./project-users.resolver";

@Module({
  imports: [ProjectUsersResolver],
})
export class ProjectUsersModule {}
