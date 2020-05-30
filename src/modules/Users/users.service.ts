import { Inject, Injectable } from "@nestjs/common";
import { DatastoreProvider } from "src/providers/datastore/datastore.provider";
import { CreateUserArg } from "./args/create-user";
import cuid from "cuid";

@Injectable()
export class UsersService {
  constructor(
    @Inject(DatastoreProvider) private datastore: DatastoreProvider
  ) {}
  private key = "Users";

  async create(user: CreateUserArg) {
    const key = this.datastore.key([this.key, cuid()]);

    await this.datastore.insert({
      key,
      data: {
        ...user,
        projectIds: [],
        groupIds: [],
        activeTask: null,
      },
    });
  }

  async getOne(id: string) {
    const path = this.datastore.key([this.key, id]);
    const [user] = await this.datastore.get(path);
    return user;
  }

  async getList() {
    const [users] = await this.datastore.createQuery(this.key).run();
    return users;
  }
}
