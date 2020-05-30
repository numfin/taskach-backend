import { Paginated } from "src/tools/paginated";
import { User } from "./user";
import { ObjectType } from "@nestjs/graphql";

@ObjectType()
export class UserPaginated extends Paginated(User) {}
