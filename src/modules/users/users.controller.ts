import { Controller } from "src/tools/controller";
import { CreateUserDTO, GetUserDTO } from "./dto/get-users.dto";
import { Parse } from "src/tools/parse";
import { Logger } from "src/logger";
import { UsersService } from "./users.service";
import { ErrorType } from "src/errors/ErrorType";

export class UsersModule extends Controller {
  logger = new Logger("UsersModule");
  user = new UsersService();

  @Parse(CreateUserDTO)
  async create(user: CreateUserDTO) {
    this.user.create(user);
    this.send();
  }

  @Parse(GetUserDTO)
  async get({ id }: GetUserDTO) {
    const user = await this.user.getOne(id);
    if (user) {
      return user;
    }
    this.error(ErrorType.NotFound);
  }

  async getList() {
    return this.user.getList();
  }
}
