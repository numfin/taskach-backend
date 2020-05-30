import { User } from "./models/user";
import { UsersService } from "./users.service";
import { Inject } from "@nestjs/common";
import { Resolver, Query, Args, ObjectType } from "@nestjs/graphql";
import { Paginated } from "src/tools/paginated";
import { CreateUserArg } from "./args/create-user";
import { UserPaginated } from "./models/user-paginated";

@Resolver(() => User)
export class UsersResolver {
  constructor(@Inject(UsersService) private user: UsersService) {}

  @Query(() => User)
  async create(@Args() user: CreateUserArg) {
    await this.user.create(user);
  }

  @Query(() => User)
  async get(@Args("id") id: string) {
    const user = await this.user.getOne(id);
    if (user) {
      return user;
    }
  }

  @Query(() => UserPaginated)
  async getList() {
    return this.user.getList();
  }
}
