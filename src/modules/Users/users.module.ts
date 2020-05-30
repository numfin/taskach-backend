import { Module } from "@nestjs/common";
import { UsersService } from "./users.service";
import { UsersResolver } from "./users.resolver";
import { DatastoreModule } from "src/providers/datastore/datastore.module";

@Module({
  imports: [DatastoreModule],
  providers: [UsersService, UsersResolver],
})
export class UsersModule {}
