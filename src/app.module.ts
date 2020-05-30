import { Module } from "@nestjs/common";
import { GraphqlModule } from "./providers/graphql/graphql.module";

import { UsersModule } from "src/modules/Users/users.module";
import { ProjectUsersModule } from "src/modules/ProjectUsers/project-users.module";
import { ProjectsModule } from "src/modules/Projects/projects.module";
import { DatastoreModule } from "./providers/datastore/datastore.module";

@Module({
  imports: [
    DatastoreModule,
    UsersModule,
    ProjectUsersModule,
    ProjectsModule,
    GraphqlModule,
  ],
})
export class AppModule {}
