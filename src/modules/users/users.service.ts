import { datastore } from "src/providers/firestore";
import { CreateUserDTO } from "./dto/get-users.dto";
import cuid from "cuid";

export class UsersService {
  private key = "Users";

  async create(user: CreateUserDTO) {
    const key = datastore.key([this.key, cuid()]);

    await datastore.insert({
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
    const path = datastore.key([this.key, id]);
    const [user] = await datastore.get(path);
    return user;
  }

  async getList() {
    const [users] = await datastore.createQuery(this.key).run();
    return users;
  }
}
