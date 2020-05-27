import { IsString, IsUUID } from "class-validator";

export class CreateUserDTO {
  @IsString()
  name: string;
  @IsString()
  surname: string;
}

export class GetUserDTO {
  @IsString()
  id: string;
}
