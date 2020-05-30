import { ObjectType, Field, Int } from "@nestjs/graphql";
import { Type } from "@nestjs/common";

export function Paginated<T>(classRef: Type<T>): any {
  @ObjectType({ isAbstract: true })
  abstract class PaginatedDTO {
    @Field(() => [classRef])
    items: T[];

    @Field(() => Int)
    total: number;
  }

  return PaginatedDTO;
}
