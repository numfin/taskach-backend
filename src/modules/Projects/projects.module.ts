import { Module } from "@nestjs/common";
import { ProjectResolver } from "./projects.resolver";

@Module({
  imports: [ProjectResolver],
})
export class ProjectsModule {}
