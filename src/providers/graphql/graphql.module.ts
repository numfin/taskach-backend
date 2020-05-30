import { GraphQLModule } from "@nestjs/graphql";
import { resolve } from "path";

export const GraphqlModule = GraphQLModule.forRoot({
  autoSchemaFile: resolve(process.cwd(), "schema.gql"),
  playground: true,
});
